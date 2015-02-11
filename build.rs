extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library("libsiphash.a",
                         &gcc::Config {
                             flags: vec!["-O3".to_string()],
                             ..Default::default()
                         },
                         &["siphash24.c"]);
}