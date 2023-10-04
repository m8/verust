extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("libverona").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=verona");
    println!("cargo:rustc-link-lib=static=stdc++");
    // println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=atomic");
}
