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


# Unsupported target "all" with type "test" omitted
# Unsupported target "entry" with type "test" omitted
# Unsupported target "extract_file" with type "example" omitted
# Unsupported target "list" with type "example" omitted
# Unsupported target "raw_list" with type "example" omitted
rust_library(
    name = "tar",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/filetime-0.1.14:filetime",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/xattr-0.1.11:xattr",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
        "xattr",
    ],
)
# Unsupported target "write" with type "example" omitted
