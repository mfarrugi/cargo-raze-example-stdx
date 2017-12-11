This is a sample project to test my fork of [`rules_rust`](https://github.com/mfarrugi/rules_rust/tree/merged-dylibs-and-raze) with [`cargo-raze`](https://github.com/acmcarther/cargo-raze).

The main point of interest here is dynamically linking the system openssl and overriding the relevant `build.rs` to do so.

There's also an example of a custom toolchain configuration, across `WORKSPACE` and `bazel/custom_rust_toolchain.bzl`.
