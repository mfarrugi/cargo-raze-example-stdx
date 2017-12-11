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


rust_library(
    name = "schannel",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/advapi32-sys-0.2.0:advapi32_sys",
        "//cargo/vendor/crypt32-sys-0.2.0:crypt32_sys",
        "//cargo/vendor/kernel32-sys-0.2.2:kernel32_sys",
        "//cargo/vendor/lazy_static-1.0.0:lazy_static",
        "//cargo/vendor/secur32-sys-0.2.0:secur32_sys",
        "//cargo/vendor/winapi-0.2.8:winapi",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
