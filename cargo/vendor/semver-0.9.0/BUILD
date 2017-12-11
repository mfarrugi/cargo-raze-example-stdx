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


# Unsupported target "deprecation" with type "test" omitted
# Unsupported target "regression" with type "test" omitted
rust_library(
    name = "semver",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/semver-parser-0.7.0:semver_parser",
        "//cargo/vendor/serde-1.0.24:serde",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
        "serde",
    ],
)
# Unsupported target "serde" with type "test" omitted
