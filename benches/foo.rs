#![feature(sip_hash_13, test)]

extern crate test;
extern crate siphash_bench;

macro_rules! benches {
    ($f:expr, $var:ident) => {
        use std::iter::repeat;
        #[bench]
        fn str_small(b: &mut ::test::Bencher) {
            let $var = "foo";
            b.iter(|| $f);
        }

        #[bench]
        fn str_medium(b: &mut ::test::Bencher) {
            let s = repeat('a').take(500).collect::<String>();
            let $var = &*s;
            b.iter(|| $f);
        }

        #[bench]
        fn str_long(b: &mut ::test::Bencher) {
            let s = repeat('a').take(10000).collect::<String>();
            let $var = &*s;
            b.iter(|| $f);
        }
    }
}

mod c1_siphash24 {
    extern {
        fn siphash(out: *mut u8, input: *const u8, len: u64, k: *const u8) -> i32;
    }

    pub fn hash(b: &[u8]) -> u64 {
        let mut ret = 0u64;
        let keys = [0u64, 0u64];
        unsafe {
            siphash(&mut ret as *mut _ as *mut _,
                    b.as_ptr(),
                    b.len() as u64,
                    keys.as_ptr() as *const _);
        }
        ret
    }

    benches!(hash(b.as_bytes()), b);
}

mod c2_siphash24 {
    extern crate libc;
    extern {
        fn siphash24(input: *const u8, sz: libc::c_ulong,
                     key: *const u8) -> u64;
    }

    pub fn hash(b: &[u8]) -> u64 {
        let keys = [0u64, 0u64];
        unsafe {
            siphash24(b.as_ptr(),
                      b.len() as libc::c_ulong,
                      keys.as_ptr() as *const _)
        }
    }

    benches!(hash(b.as_bytes()), b);
}

mod cpp_siphash {
    extern crate libc;

    extern {
        fn SipHashC(keys: *const u64,
                    bytes: *const u8,
                    size: u64) -> u64;
    }

    pub fn hash(b: &[u8]) -> u64 {
        let keys = [0u64, 0u64];
        unsafe {
            SipHashC(keys.as_ptr(),
                     b.as_ptr(),
                     b.len() as libc::c_ulong)
        }
    }

    benches!(hash(b.as_bytes()), b);
}

mod rust_siphash24 {
    use std::hash::{SipHasher, Hasher};

    pub fn hash(b: &[u8]) -> u64 {
        let mut s = SipHasher::new_with_keys(0, 0);
        s.write(b);
        s.finish()
    }

    benches!(hash(b.as_bytes()), b);
}

mod rust_siphash13 {
    use std::hash::{SipHasher13, Hasher};

    pub fn hash(b: &[u8]) -> u64 {
        let mut s = SipHasher13::new_with_keys(0, 0);
        s.write(b);
        s.finish()
    }

    benches!(hash(b.as_bytes()), b);
}

#[test]
fn test_same() {
    assert_eq!(c1_siphash24::hash(&[1, 2, 3, 4]),
               rust_siphash24::hash(&[1, 2, 3, 4]));
}
