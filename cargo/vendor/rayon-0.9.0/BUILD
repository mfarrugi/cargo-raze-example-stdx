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


# Unsupported target "clones" with type "test" omitted
# Unsupported target "cpu_monitor" with type "example" omitted
# Unsupported target "debug" with type "test" omitted
# Unsupported target "intersperse" with type "test" omitted
# Unsupported target "producer_split_at" with type "test" omitted
rust_library(
    name = "rayon",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/either-1.4.0:either",
        "//cargo/vendor/rayon-core-1.3.0:rayon_core",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
