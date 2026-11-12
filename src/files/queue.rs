//! A queue of small jobs run on threads of their own.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};

type Job<T> = (T, Arc<AtomicBool>);

pub struct Queue<T: Send + 'static> {
    send: Option<mpsc::Sender<Job<T>>>,
    round: Mutex<Arc<AtomicBool>>,
}

impl<T: Send + 'static> Queue<T> {
    pub fn new(name: &str, workers: usize, run: impl Fn(T, &AtomicBool) + Send + Sync + 'static) -> Queue<T> {
        let (send, take) = mpsc::channel::<Job<T>>();
        let take = Arc::new(Mutex::new(take));
        let run = Arc::new(run);
        for n in 0..workers.max(1) {
            let (take, run) = (take.clone(), run.clone());
            let spawned = crate::common::gentle::spawn(&format!("{name}-{n}"), move || loop {
                let next = take.lock().map(|t| t.recv());
                let Ok(Ok((job, stop))) = next else { break };
                if !stop.load(Ordering::Relaxed) {
                    run(job, &stop);
                }
            });
            if let Err(e) = spawned {
                eprintln!("trindle: files: {name}: {e}");
            }
        }
        Queue { send: Some(send), round: Mutex::new(Arc::new(AtomicBool::new(false))) }
    }

    pub fn ask(&self, job: T) {
        let stop = self.round.lock().map(|r| r.clone()).unwrap_or_default();
        if let Some(send) = &self.send {
            let _ = send.send((job, stop));
        }
    }

    pub fn cancel(&self) {
        if let Ok(mut round) = self.round.lock() {
            round.store(true, Ordering::Relaxed);
            *round = Arc::new(AtomicBool::new(false));
        }
    }
}

impl<T: Send + 'static> Drop for Queue<T> {
    fn drop(&mut self) {
        self.cancel();
        self.send = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn jobs_are_done_and_cancelled_ones_are_not() {
        let (tx, rx) = mpsc::channel();
        let gate = Arc::new(Mutex::new(()));
        let held = gate.lock().unwrap();
        let g = gate.clone();
        let q = Queue::new("test-queue", 1, move |n: u32, stop: &AtomicBool| {
            drop(g.lock().unwrap());
            if !stop.load(Ordering::Relaxed) {
                tx.send(n).unwrap();
            }
        });
        q.ask(1);
        q.ask(2);
        q.cancel();
        q.ask(3);
        drop(held);
        assert_eq!(rx.recv_timeout(Duration::from_secs(5)), Ok(3));
        assert!(rx.recv_timeout(Duration::from_millis(100)).is_err());
    }
}
