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


# Unsupported target "bad" with type "test" omitted
# Unsupported target "download-rust-lang" with type "example" omitted
# Unsupported target "google" with type "test" omitted
# Unsupported target "hyper-client" with type "example" omitted
# Unsupported target "hyper-server" with type "example" omitted
# Unsupported target "smoke" with type "test" omitted
rust_library(
    name = "tokio_tls",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/futures-0.1.17:futures",
        "//cargo/vendor/native-tls-0.1.4:native_tls",
        "//cargo/vendor/tokio-core-0.1.10:tokio_core",
        "//cargo/vendor/tokio-io-0.1.4:tokio_io",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
