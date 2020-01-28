#![feature(test)]

extern crate siphash_bench;
extern crate test;

const KEYS: [u64; 2] = [1316170652228047548, 9438327870879860736];
const U256: [u8; 32] = [
    74u8, 56, 153, 219, 30, 49, 85, 215, 147, 91, 131, 134, 46, 229, 143, 243, 206, 124, 14, 217,
    150, 25, 64, 152, 165, 129, 227, 53, 159, 141, 61, 35,
];
const U256_EXTRA: [u8; 36] = [
    74u8, 56, 153, 219, 30, 49, 85, 215, 147, 91, 131, 134, 46, 229, 143, 243, 206, 124, 14, 217,
    150, 25, 64, 152, 165, 129, 227, 53, 159, 141, 61, 35, 70, 184, 192, 170,
];
use std::iter::repeat;
use std::os::raw::c_ulong;

macro_rules! benches {
    ($f:expr, $var:ident) => {
        #[bench]
        fn str_small(b: &mut ::test::Bencher) {
            let $var = b"foo";
            b.iter(|| $f(&$var[..]));
        }

        #[bench]
        fn str_256(b: &mut ::test::Bencher) {
            let $var = U256.to_vec();
            b.iter(|| $f(&$var[..]));
        }
        #[bench]
        fn str_256_extra(b: &mut ::test::Bencher) {
            let $var = U256_EXTRA.to_vec();
            b.iter(|| $f(&$var[..]));
        }

        #[bench]
        fn str_medium(b: &mut ::test::Bencher) {
            let s = repeat('a').take(500).collect::<String>().into_bytes();
            let $var = &*s;
            b.iter(|| $f(&$var[..]));
        }

        #[bench]
        fn str_long(b: &mut ::test::Bencher) {
            let s = repeat('a').take(10000).collect::<String>().into_bytes();
            let $var = &*s;
            b.iter(|| $f(&$var[..]));
        }
    };
}

mod siphash24c {
    use super::*;
    extern "C" {
        fn siphash(out: *mut u8, input: *const u8, len: u64, k: *const u8) -> i32;
    }

    #[inline(always)]
    pub fn hash(b: &[u8]) -> u64 {
        let mut ret = 0u64;
        unsafe {
            siphash(
                &mut ret as *mut _ as *mut _,
                b.as_ptr(),
                b.len() as u64,
                KEYS.as_ptr() as *const _,
            );
        }
        ret
    }

    benches!(hash, b);
}

mod csiphashc {
    use super::*;
    extern "C" {
        fn siphash24(input: *const u8, sz: c_ulong, key: *const u8) -> u64;
    }

    #[inline(always)]
    pub fn hash(b: &[u8]) -> u64 {
        unsafe { siphash24(b.as_ptr(), b.len() as c_ulong, KEYS.as_ptr() as *const _) }
    }

    benches!(hash, b);
}

mod cpp_siphash24 {
    use super::*;

    extern "C" {
        fn SipHashC(keys: *const u64, bytes: *const u8, size: u64) -> u64;
    }

    #[inline(always)]
    pub fn hash(b: &[u8]) -> u64 {
        unsafe { SipHashC(KEYS.as_ptr(), b.as_ptr(), b.len() as c_ulong) }
    }

    benches!(hash, b);
}

mod cpp_bitcoincore_siphash24 {
    use super::*;
    use bitcoin_hashes::siphash24::Hash;

    extern "C" {
        fn SipHashUint256C(k0: u64, k1: u64, val: *const u8) -> u64;
        fn SipHashUint256ExtraC(k0: u64, k1: u64, val: *const u8) -> u64;
        fn SipHashNormal(k0: u64, k1: u64, val: *const u8, size: usize) -> u64;
    }

    #[inline(always)]
    pub fn hash_256(d: [u8; 32]) -> u64 {
        unsafe { SipHashUint256C(KEYS[0], KEYS[1], d.as_ptr()) }
    }

    #[inline(always)]
    pub fn hash_256_extra(d: [u8; 36]) -> u64 {
        unsafe { SipHashUint256ExtraC(KEYS[0], KEYS[1], d.as_ptr()) }
    }

    #[inline(always)]
    pub fn hash(b: &[u8]) -> u64 {
        unsafe { SipHashNormal(KEYS[0], KEYS[1], b.as_ptr(), b.len()) }
    }
    benches!(hash, b);

    #[bench]
    fn str_256_specialized(b: &mut ::test::Bencher) {
        let val = [
            74u8, 56, 153, 219, 30, 49, 85, 215, 147, 91, 131, 134, 46, 229, 143, 243, 206, 124,
            14, 217, 150, 25, 64, 152, 165, 129, 227, 53, 159, 141, 61, 35,
        ];
        b.iter(|| hash_256(val));
    }
    #[bench]
    fn str_256_extra_specialized(b: &mut ::test::Bencher) {
        let val = [
            74u8, 56, 153, 219, 30, 49, 85, 215, 147, 91, 131, 134, 46, 229, 143, 243, 206, 124,
            14, 217, 150, 25, 64, 152, 165, 129, 227, 53, 159, 141, 61, 35, 70, 184, 192, 170,
        ];
        b.iter(|| hash_256_extra(val));
    }
}

mod rust_siphash24 {
    use super::*;
    use std::hash::{Hasher, SipHasher};

    #[inline(always)]
    pub fn hash(b: &[u8]) -> u64 {
        let mut s = SipHasher::new_with_keys(KEYS[0], KEYS[1]);
        s.write(b);
        s.finish()
    }

    benches!(hash, b);
}

mod rust_bitcoinhashes_siphash24 {
    use super::*;
    use bitcoin_hashes::siphash24::Hash;

    #[inline(always)]
    pub fn hash(b: &[u8]) -> u64 {
        Hash::hash_to_u64_with_keys(KEYS[0], KEYS[1], b)
    }

    benches!(hash, b);
}

#[test]
fn test_same() {
    test(&[1, 2, 3, 4]);
    test(&repeat('a').take(500).collect::<String>().into_bytes());
    test(&U256);
    test(&U256_EXTRA);
}

fn test(data: &[u8]) {
    let rust = rust_siphash24::hash(data);
    assert_eq!(siphash24c::hash(data), rust);
    assert_eq!(cpp_siphash24::hash(data), rust);
    assert_eq!(siphash24c::hash(data), rust);
    assert_eq!(rust_bitcoinhashes_siphash24::hash(data), rust);
    assert_eq!(cpp_bitcoincore_siphash24::hash(data), rust);
}

#[test]
fn test_same_256_extra() {
    let rust = rust_siphash24::hash(&U256_EXTRA[..]);
    assert_eq!(cpp_bitcoincore_siphash24::hash_256_extra(U256_EXTRA), rust);
}
