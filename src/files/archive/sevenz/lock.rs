// 7z's lock: the AES key made from a password and the reader that opens a block.

use std::io::{self, Read};

use aes::cipher::{BlockDecrypt, KeyInit};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::files::archive::sevenz::{sz_bad, sz_locked};
use crate::files::archive::trouble::Key;
use crate::toolkit::t;

const SZ_STIR_MOST: u8 = 24;

pub(in crate::files::archive) fn sz_key(password: &str, power: u8, salt: &[u8]) -> io::Result<Zeroizing<[u8; 32]>> {
    let mut typed = Zeroizing::new(Vec::with_capacity(password.len() * 2));
    for unit in password.encode_utf16() {
        typed.extend_from_slice(&unit.to_le_bytes());
    }
    let mut key = Zeroizing::new([0u8; 32]);
    if power == 0x3f {
        for (k, b) in key.iter_mut().zip(salt.iter().chain(typed.iter())) {
            *k = *b;
        }
        return Ok(key);
    }
    if power > SZ_STIR_MOST {
        return Err(sz_bad(t!("folders-archive-7z-a-lock-too-slow-to-open")));
    }
    let mut sha = Sha256::new();
    for round in 0..1u64 << power {
        sha.update(salt);
        sha.update(typed.as_slice());
        sha.update(round.to_le_bytes());
    }
    key.copy_from_slice(&sha.finalize());
    Ok(key)
}

type SzMade = (u8, Vec<u8>, Zeroizing<[u8; 32]>);

pub(in crate::files::archive) struct SzKeys<'k> {
    pub(in crate::files::archive) key: Option<&'k Key>,
    pub(in crate::files::archive) made: std::cell::RefCell<Vec<SzMade>>,
}

impl<'k> SzKeys<'k> {
    pub(in crate::files::archive) fn new(key: Option<&'k Key>) -> SzKeys<'k> {
        SzKeys { key, made: Default::default() }
    }

    fn of(&self, power: u8, salt: &[u8]) -> io::Result<Zeroizing<[u8; 32]>> {
        let key = self.key.ok_or_else(sz_locked)?;
        if let Some((_, _, made)) = self.made.borrow().iter().find(|(p, s, _)| *p == power && s == salt) {
            return Ok(made.clone());
        }
        let made = sz_key(key.text(), power, salt)?;
        self.made.borrow_mut().push((power, salt.to_vec(), made.clone()));
        Ok(made)
    }
}

const SZ_AES_CHUNK: usize = 1 << 14;

pub(super) struct SzAes<R> {
    pub(super) inner: R,
    pub(super) cipher: aes::Aes256,
    pub(super) last: [u8; 16],
    pub(super) buf: Vec<u8>,
    pub(super) at: usize,
}

impl<R: Read> SzAes<R> {
    pub(super) fn open(inner: R, props: &[u8], keys: &SzKeys) -> io::Result<SzAes<R>> {
        let first = *props.first().ok_or_else(|| sz_bad(t!("folders-archive-7z-a-lock-with-no-settings")))?;
        let (mut salt_len, mut iv_len) = (0, 0);
        if first & 0xc0 != 0 {
            let second = *props.get(1).ok_or_else(|| sz_bad(t!("folders-archive-7z-a-lock-with-short-settings")))?;
            salt_len = usize::from((first >> 7) & 1) + usize::from(second >> 4);
            iv_len = usize::from((first >> 6) & 1) + usize::from(second & 0x0f);
        }
        let rest = props.get(2..2 + salt_len + iv_len).filter(|_| salt_len + iv_len > 0).unwrap_or(&[]);
        if rest.len() != salt_len + iv_len {
            return Err(sz_bad(t!("folders-archive-7z-a-lock-with-short-settings")));
        }
        let (salt, iv) = rest.split_at(salt_len);
        let mut last = [0u8; 16];
        last[..iv_len].copy_from_slice(iv);
        let key = keys.of(first & 0x3f, salt)?;
        let cipher = aes::Aes256::new_from_slice(key.as_slice()).map_err(|_| sz_bad(t!("folders-archive-7z-a-lock-with-no-key")))?;
        Ok(SzAes { inner, cipher, last, buf: Vec::new(), at: 0 })
    }
}

impl<R: Read> Read for SzAes<R> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.at == self.buf.len() {
            self.buf.resize(SZ_AES_CHUNK, 0);
            self.at = 0;
            let mut got = 0;
            while got < self.buf.len() {
                match self.inner.read(&mut self.buf[got..]) {
                    Ok(0) => break,
                    Ok(n) => got += n,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => {
                        self.buf.clear();
                        return Err(e);
                    }
                }
            }
            self.buf.truncate(got);
            if got % 16 != 0 {
                self.buf.clear();
                return Err(sz_bad(t!("folders-archive-7z-a-locked-block-is-not-whole")));
            }
            for block in self.buf.chunks_exact_mut(16) {
                let mut held = [0u8; 16];
                held.copy_from_slice(block);
                self.cipher.decrypt_block(aes::Block::from_mut_slice(block));
                for (b, l) in block.iter_mut().zip(self.last) {
                    *b ^= l;
                }
                self.last = held;
            }
        }
        let n = out.len().min(self.buf.len() - self.at);
        out[..n].copy_from_slice(&self.buf[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}
