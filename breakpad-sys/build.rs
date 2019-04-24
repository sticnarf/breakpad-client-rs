use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

fn main() -> io::Result<()> {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .enable_cxx_namespaces()
        .clang_args(&["-x", "c++", "-I", "breakpad/src"])
        .header("breakpad_c.h")
        .header("breakpad/src/client/linux/handler/exception_handler.h")
        .whitelist_type("google_breakpad::MinidumpDescriptor")
        .whitelist_type("google_breakpad::ExceptionHandler")
        .whitelist_function("register_handler_from_path")
        .whitelist_function("register_handler_from_fd")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(true)
        .include("breakpad/src")
        .file("breakpad_c.cpp")
        .compile("breakpad_c.a");

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

    println!("cargo:rustc-link-lib=static=breakpad");
    println!("cargo:rustc-link-lib=static=breakpad_c");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("src/").to_str().unwrap()
    );
    println!("cargo:rustc-link-search=native={}", dst.to_str().unwrap());

    Ok(())
}
