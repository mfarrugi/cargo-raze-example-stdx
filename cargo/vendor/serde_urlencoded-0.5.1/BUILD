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
    name = "serde_urlencoded",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/dtoa-0.4.2:dtoa",
        "//cargo/vendor/itoa-0.3.4:itoa",
        "//cargo/vendor/serde-1.0.24:serde",
        "//cargo/vendor/url-1.6.0:url",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "test_deserialize" with type "test" omitted
# Unsupported target "test_serialize" with type "test" omitted
