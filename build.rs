extern crate cc;

fn main() {
    let mut conf_base = cc::Build::new();
    conf_base.flag("-O3").flag("-Wno-implicit-fallthrough");
    // .flag("-march=native");

    conf_base
        .clone()
        .file("siphash24.c")
        .compile("libsiphash.a");
    conf_base
        .clone()
        .file("csiphash.c")
        .compile("libsiphash2.a");

    let mut cpp_conf = conf_base;
    cpp_conf.cpp(true).flag("-std=c++11");

    cpp_conf
        .clone()
        .file("highwayhash/highwayhash/sip_hash.cc")
        .include("highwayhash")
        .flag("-mavx2")
        .compile("libcppsiphash.a");

    cpp_conf
        .file("bitcoin_siphash_wrapper.cpp")
        .include("bitcoin/src")
        .compile("libbitcoinsiphash.a");
}
