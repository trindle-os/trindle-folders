//! Zip, unlocked: its first lock (ZipCrypto) and WinZip's AES.

use std::io::{self, Read};

use aes::cipher::{BlockEncrypt, KeyInit};
use zeroize::{Zeroize, Zeroizing};

use crate::files::archive::trouble::{trouble, Key, Trouble};
use crate::files::archive::zip::bad;
use crate::toolkit::t;

const CRC_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
};

fn crc_step(crc: u32, b: u8) -> u32 {
    CRC_TABLE[((crc ^ u32::from(b)) & 0xff) as usize] ^ (crc >> 8)
}

pub(in crate::files::archive) struct ZipOld<R> {
    pub(in crate::files::archive) inner: R,
    pub(in crate::files::archive) keys: [u32; 3],
}

impl<R: Read> ZipOld<R> {
    pub(super) fn open(inner: R, key: &Key, check: [u8; 2]) -> io::Result<ZipOld<R>> {
        let mut z = ZipOld { inner, keys: [0x1234_5678, 0x2345_6789, 0x3456_7890] };
        for &b in key.text().as_bytes() {
            z.stir(b);
        }
        let mut start = [0u8; 12];
        z.inner.read_exact(&mut start).map_err(|_| bad(t!("folders-archive-zip-a-locked-entry-is-cut-short")))?;
        z.unlock(&mut start);
        if !check.contains(&start[11]) {
            return Err(trouble(Trouble::WrongKey));
        }
        Ok(z)
    }

    pub(in crate::files::archive) fn stir(&mut self, b: u8) {
        let k = &mut self.keys;
        k[0] = crc_step(k[0], b);
        k[1] = k[1].wrapping_add(k[0] & 0xff).wrapping_mul(134_775_813).wrapping_add(1);
        k[2] = crc_step(k[2], (k[1] >> 24) as u8);
    }

    fn unlock(&mut self, bytes: &mut [u8]) {
        for b in bytes {
            let t = (self.keys[2] | 2) & 0xffff;
            *b ^= (t.wrapping_mul(t ^ 1) >> 8) as u8;
            self.stir(*b);
        }
    }
}

impl<R: Read> Read for ZipOld<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.unlock(&mut buf[..n]);
        Ok(n)
    }
}

impl<R> Drop for ZipOld<R> {
    fn drop(&mut self) {
        self.keys.zeroize();
    }
}

enum AesKey {
    Short(aes::Aes128),
    Middle(aes::Aes192),
    Long(aes::Aes256),
}

impl AesKey {
    fn new(key: &[u8]) -> io::Result<AesKey> {
        let wrong = |_| bad(t!("folders-archive-zip-a-lock-with-a-key-of-no-known-length"));
        Ok(match key.len() {
            16 => AesKey::Short(aes::Aes128::new_from_slice(key).map_err(wrong)?),
            24 => AesKey::Middle(aes::Aes192::new_from_slice(key).map_err(wrong)?),
            _ => AesKey::Long(aes::Aes256::new_from_slice(key).map_err(wrong)?),
        })
    }

    fn scramble(&self, block: &mut [u8; 16]) {
        let block = aes::Block::from_mut_slice(block);
        match self {
            AesKey::Short(c) => c.encrypt_block(block),
            AesKey::Middle(c) => c.encrypt_block(block),
            AesKey::Long(c) => c.encrypt_block(block),
        }
    }
}

const ZIP_AES_ROUNDS: u32 = 1000;
const ZIP_AES_SEAL: usize = 10;

pub(super) struct ZipUnlock<R> {
    inner: R,
    left: u64,
    cipher: AesKey,
    counter: u128,
    pad: [u8; 16],
    used: usize,
    seal: Option<ring::hmac::Context>,
}

impl<R: Read> ZipUnlock<R> {
    pub(super) fn open(mut inner: R, packed: u64, strength: u8, key: &Key) -> io::Result<ZipUnlock<R>> {
        let (salt_len, key_len) = match strength {
            1 => (8, 16),
            2 => (12, 24),
            _ => (16, 32),
        };
        let left = packed.checked_sub((salt_len + 2 + ZIP_AES_SEAL) as u64).ok_or_else(|| bad(t!("folders-archive-zip-a-locked-entry-is-cut-short")))?;
        let mut start = [0u8; 18];
        let start = &mut start[..salt_len + 2];
        inner.read_exact(start).map_err(|_| bad(t!("folders-archive-zip-a-locked-entry-is-cut-short")))?;
        let (salt, fits) = start.split_at(salt_len);
        let mut made = Zeroizing::new([0u8; 66]);
        let made = &mut made[..2 * key_len + 2];
        let rounds = std::num::NonZeroU32::new(ZIP_AES_ROUNDS).unwrap_or(std::num::NonZeroU32::MIN);
        ring::pbkdf2::derive(ring::pbkdf2::PBKDF2_HMAC_SHA1, rounds, salt, key.text().as_bytes(), made);
        if made[2 * key_len..] != *fits {
            return Err(trouble(Trouble::WrongKey));
        }
        let seal = ring::hmac::Context::with_key(&ring::hmac::Key::new(ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &made[key_len..2 * key_len]));
        Ok(ZipUnlock { inner, left, cipher: AesKey::new(&made[..key_len])?, counter: 0, pad: [0; 16], used: 16, seal: Some(seal) })
    }
}

impl<R: Read> Read for ZipUnlock<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.left == 0 {
            if let Some(seal) = self.seal.take() {
                let mut said = [0u8; ZIP_AES_SEAL];
                self.inner.read_exact(&mut said).map_err(|_| bad(t!("folders-archive-zip-a-locked-entry-is-cut-short")))?;
                if seal.sign().as_ref()[..ZIP_AES_SEAL] != said {
                    return Err(bad(t!("folders-archive-zip-a-locked-entry-s-check-does-not-match")));
                }
            }
            return Ok(0);
        }
        let room = (buf.len() as u64).min(self.left) as usize;
        let n = self.inner.read(&mut buf[..room])?;
        if n == 0 {
            return Err(bad(t!("folders-archive-zip-a-locked-entry-is-cut-short")));
        }
        if let Some(seal) = &mut self.seal {
            seal.update(&buf[..n]);
        }
        for b in &mut buf[..n] {
            if self.used == 16 {
                self.counter += 1;
                self.pad = self.counter.to_le_bytes();
                self.cipher.scramble(&mut self.pad);
                self.used = 0;
            }
            *b ^= self.pad[self.used];
            self.used += 1;
        }
        self.left -= n as u64;
        Ok(n)
    }
}

impl<R> Drop for ZipUnlock<R> {
    fn drop(&mut self) {
        self.pad.zeroize();
    }
}
