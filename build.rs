extern crate gcc;

fn main() {
    gcc::Config::new().file("siphash24.c").flag("-O3").compile("libsiphash.a");
    gcc::Config::new().file("csiphash.c").flag("-O3").compile("libsiphash2.a");
}
