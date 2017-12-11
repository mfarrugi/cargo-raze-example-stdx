"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//cargo:__subpackages__"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


# Unsupported target "backtrace" with type "example" omitted
rust_library(
    name = "backtrace",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/backtrace-sys-0.1.16:backtrace_sys",
        "//cargo/vendor/cfg-if-0.1.2:cfg_if",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/rustc-demangle-0.1.5:rustc_demangle",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "backtrace-sys",
        "coresymbolication",
        "dbghelp",
        "dbghelp-sys",
        "default",
        "dladdr",
        "kernel32-sys",
        "libbacktrace",
        "libunwind",
        "winapi",
    ],
)
# Unsupported target "long_fn_name" with type "test" omitted
# Unsupported target "raw" with type "example" omitted
# Unsupported target "smoke" with type "test" omitted
