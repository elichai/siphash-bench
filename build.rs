extern crate gcc;

fn main() {
    gcc::Config::new().file("siphash24.c").flag("-O3").compile("libsiphash.a");
    gcc::Config::new().file("csiphash.c").flag("-O3").compile("libsiphash2.a");
    gcc::Config::new()
        .file("highwayhash/highwayhash/sip_hash.cc")
        .include("highwayhash")
        .cpp(true)
        .flag("-O3")
        .flag("-std=c++11")
        .compile("libcppsiphash.a");
}
