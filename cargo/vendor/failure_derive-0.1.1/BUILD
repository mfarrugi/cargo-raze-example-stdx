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


# Unsupported target "backtrace" with type "test" omitted
rust_library(
    name = "failure_derive",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/quote-0.3.15:quote",
        "//cargo/vendor/syn-0.11.11:syn",
        "//cargo/vendor/synstructure-0.6.1:synstructure",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
        "std",
    ],
)
# Unsupported target "no_derive_display" with type "test" omitted
# Unsupported target "tests" with type "test" omitted
# Unsupported target "wraps" with type "test" omitted
