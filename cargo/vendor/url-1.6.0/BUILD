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


# Unsupported target "data" with type "test" omitted
# Unsupported target "unit" with type "test" omitted
rust_library(
    name = "url",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/idna-0.1.4:idna",
        "//cargo/vendor/matches-0.1.6:matches",
        "//cargo/vendor/percent-encoding-1.0.1:percent_encoding",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
