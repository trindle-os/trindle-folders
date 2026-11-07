// How long work on files says how far it has got, and how it is stopped.

use std::cell::Cell;
use std::io::{self, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub const GAP: Duration = Duration::from_millis(100);

#[derive(Debug)]
struct Stopped;

impl std::fmt::Display for Stopped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("stopped")
    }
}

impl std::error::Error for Stopped {}

pub fn stopped() -> io::Error {
    io::Error::other(Stopped)
}

pub fn is_stopped(e: &io::Error) -> bool {
    e.get_ref().is_some_and(|inner| inner.is::<Stopped>())
}

pub struct Meter<'a> {
    stop: &'a AtomicBool,
    tell: &'a dyn Fn(u64, u64),
    total: Cell<u64>,
    done: Cell<u64>,
    last: Cell<Option<Instant>>,
}

impl<'a> Meter<'a> {
    pub fn new(stop: &'a AtomicBool, tell: &'a dyn Fn(u64, u64)) -> Meter<'a> {
        Meter { stop, tell, total: Cell::new(0), done: Cell::new(0), last: Cell::new(None) }
    }

    pub fn total(&self, total: u64) {
        self.total.set(total);
        (self.tell)(self.done.get(), total);
        self.last.set(Some(Instant::now()));
    }

    pub fn hushed(&self) -> Meter<'a> {
        fn hush(_: u64, _: u64) {}
        Meter::new(self.stop, &hush)
    }

    pub fn switch(&self) -> &'a AtomicBool {
        self.stop
    }

    #[cfg(test)]
    pub fn done(&self) -> u64 {
        self.done.get()
    }

    pub fn add(&self, n: u64) -> io::Result<()> {
        self.done.set(self.done.get() + n);
        if self.last.get().is_none_or(|t| t.elapsed() >= GAP) {
            (self.tell)(self.done.get(), self.total.get());
            self.last.set(Some(Instant::now()));
        }
        self.check()
    }

    pub fn check(&self) -> io::Result<()> {
        if self.stop.load(Ordering::Relaxed) { Err(stopped()) } else { Ok(()) }
    }

    pub fn end(&self) {
        (self.tell)(self.done.get(), self.total.get());
    }
}

pub struct Counted<'m, T> {
    pub inner: T,
    meter: &'m Meter<'m>,
}

impl<'m, T> Counted<'m, T> {
    pub fn new(inner: T, meter: &'m Meter<'m>) -> Counted<'m, T> {
        Counted { inner, meter }
    }
}

impl<T: Read> Read for Counted<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.meter.add(n as u64)?;
        Ok(n)
    }
}

impl<T: Write> Write for Counted<'_, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.meter.add(n as u64)?;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_meter_counts_tells_and_stops() {
        let stop = AtomicBool::new(false);
        let seen = std::cell::RefCell::new(Vec::new());
        let tell = |d, t| seen.borrow_mut().push((d, t));
        let meter = Meter::new(&stop, &tell);
        meter.total(10);
        let mut r = Counted::new(&b"hello"[..], &meter);
        let mut out = Vec::new();
        r.read_to_end(&mut out).unwrap();
        assert_eq!(meter.done(), 5);
        meter.end();
        assert_eq!(seen.borrow().last(), Some(&(5, 10)));
        stop.store(true, Ordering::Relaxed);
        let e = meter.add(1).unwrap_err();
        assert!(is_stopped(&e));
        assert!(!is_stopped(&io::Error::other("other")));
    }
}
