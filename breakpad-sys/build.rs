use std::path::PathBuf;
use std::{env, io};

fn main() -> io::Result<()> {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .include("breakpad/src")
        .file("breakpad_c.cpp")
        .file("breakpad/src/client/linux/crash_generation/crash_generation_client.cc")
        .file("breakpad/src/client/linux/dump_writer_common/thread_info.cc")
        .file("breakpad/src/client/linux/dump_writer_common/ucontext_reader.cc")
        .file("breakpad/src/client/linux/handler/exception_handler.cc")
        .file("breakpad/src/client/linux/handler/minidump_descriptor.cc")
        .file("breakpad/src/client/linux/log/log.cc")
        .file("breakpad/src/client/linux/microdump_writer/microdump_writer.cc")
        .file("breakpad/src/client/linux/minidump_writer/linux_core_dumper.cc")
        .file("breakpad/src/client/linux/minidump_writer/linux_dumper.cc")
        .file("breakpad/src/client/linux/minidump_writer/linux_ptrace_dumper.cc")
        .file("breakpad/src/client/linux/minidump_writer/minidump_writer.cc")
        .file("breakpad/src/client/minidump_file_writer.cc")
        .file("breakpad/src/common/convert_UTF.c")
        .file("breakpad/src/common/md5.cc")
        .file("breakpad/src/common/string_conversion.cc")
        .file("breakpad/src/common/linux/elf_core_dump.cc")
        .file("breakpad/src/common/linux/elfutils.cc")
        .file("breakpad/src/common/linux/file_id.cc")
        .file("breakpad/src/common/linux/guid_creator.cc")
        .file("breakpad/src/common/linux/linux_libc_support.cc")
        .file("breakpad/src/common/linux/memory_mapped_file.cc")
        .file("breakpad/src/common/linux/safe_readlink.cc")
        .compile("libbreakpad_c.a");

    println!("cargo:rustc-link-lib=static=breakpad_c");
    println!("cargo:rustc-link-search=native={}", dst.to_str().unwrap());

    println!("cargo:rerun-if-changed=breakpad");
    println!("cargo:rerun-if-changed=breakpad_c.cpp");
    println!("cargo:rerun-if-changed=breakpad_c.h");

    Ok(())
}
