use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

fn main() -> io::Result<()> {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());

    // configure
    if !Command::new(root.join("breakpad/configure"))
        .current_dir(&dst)
        .status()?
        .success()
    {
        panic!("configure failed");
    }

    if !Command::new("make").current_dir(&dst).status()?.success() {
        panic!("make failed");
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .include("breakpad/src")
        .file("breakpad_c.cpp")
        .compile("libbreakpad_c.a");

    println!("cargo:rustc-link-lib=static=breakpad_client");
    println!("cargo:rustc-link-lib=static=breakpad_c");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("src/client/linux").to_str().unwrap()
    );
    println!("cargo:rustc-link-search=native={}", dst.to_str().unwrap());

    println!("cargo:rerun-if-changed=breakpad");
    println!("cargo:rerun-if-changed=breakpad_c.cpp");
    println!("cargo:rerun-if-changed=breakpad_c.h");

    Ok(())
}
