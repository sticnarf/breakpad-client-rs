# breakpad-client-rs

[![CircleCI](https://circleci.com/gh/sticnarf/breakpad-client-rs.svg?style=svg)](https://circleci.com/gh/sticnarf/breakpad-client-rs)

This library helps you use [Google Breakpad](https://chromium.googlesource.com/breakpad/breakpad/)
to generate minidumps in Rust.

## Build

Clone this repo and run `git submodule update --init --recursive`.

Make sure your C++ compiler supports C++11.

## Limitations

* Linux only

* DumpOnConsole not supported

* You can get only the dump file path in the minidump callback
  instead of a MinidumpDescriptor