use std::path::Path;
use std::env;

fn main() {
    println!("cargo:rustc-link-lib=static=ta_common_cmr");
    println!("cargo:rustc-link-lib=static=ta_abstract_cmr");
    println!("cargo:rustc-link-lib=static=ta_func_cmr");
    println!("cargo:rustc-link-lib=static=ta_libc_cmr");
    //let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    //println!("cargo:rustc-link-search={}", Path::new(&dir).join("ta-lib").join("lib").display());
    println!("cargo:rustc-link-search={}", Path::new("c:\\").join("ta-lib").join("c").join("lib").display());
}