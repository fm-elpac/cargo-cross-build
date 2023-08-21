# cargo-cross-build

<https://github.com/fm-elpac/cargo-cross-build>

![build](https://github.com/fm-elpac/cargo-cross-build/actions/workflows/build.yml/badge.svg)

## Usage

This is a modified [`cargo`](https://github.com/rust-lang/cargo).

It is used for cross compile: run `build.rs` on target !! (NOT host)

1. It will build `build.rs` for target (not host):

   set environment variable `CARGO_CROSS_BUILD_CRATES` to a list of crate name,

   and set environment variable `CARGO_CROSS_BUILD_RS` to a list of the END of
   `build.rs` file path.

   all other `build.rs` will build for host (the default cargo behavior).

2. When run `build.rs`, it will not run the compiled binary
   (`build-script-build`) directly, but will run the custom command:

   set environment variable `CARGO_CROSS_BUILD_RUN` to the custom command.

   For example, cargo will run
   `target/release/build/quote-1e3314522ce020fb/build-script-build` by default,
   but now cargo will run
   `run_build.sh target/release/build/quote-1e3314522ce020fb/build-script-build`.

   Then you can use `run_build.sh` to do the actual work to run it on the target
   ! (such as QEMU docker)

Example:

```sh
export CARGO_CROSS_BUILD_CRATES=deno_runtime:deno
export CARGO_CROSS_BUILD_RS=deno_runtime-0.123.0/build.rs:deno-1.36.1/build.rs
export CARGO_CROSS_BUILD_RUN=run_build.sh
./cargo build --release --target aarch64-linux-android
```

TODO
