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
    name = "serde_derive",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/quote-0.3.15:quote",
        "//cargo/vendor/serde_derive_internals-0.18.0:serde_derive_internals",
        "//cargo/vendor/syn-0.11.11:syn",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
