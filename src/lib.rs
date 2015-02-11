#![feature(hash, core)]

use std::default::Default;
use std::hash::*;

pub struct FnvHasher(u64);

impl Default for FnvHasher {
    #[inline]
    fn default() -> FnvHasher { FnvHasher(0xcbf29ce484222325) }
}

impl Hasher for FnvHasher {
    type Output = u64;
    #[inline]
    fn reset(&mut self) { *self = Default::default(); }
    #[inline]
    fn finish(&self) -> u64 { self.0 }
}

impl Writer for FnvHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;
        for byte in bytes {
            hash = hash ^ (*byte as u64);
            hash = hash * 0x100000001b3;
        }
        *self = FnvHasher(hash);
    }
}
