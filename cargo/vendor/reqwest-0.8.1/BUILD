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


# Unsupported target "async" with type "example" omitted
# Unsupported target "async" with type "test" omitted
# Unsupported target "client" with type "test" omitted
# Unsupported target "gzip" with type "test" omitted
# Unsupported target "multipart" with type "test" omitted
# Unsupported target "proxy" with type "test" omitted
# Unsupported target "redirect" with type "test" omitted
rust_library(
    name = "reqwest",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bytes-0.4.5:bytes",
        "//cargo/vendor/futures-0.1.17:futures",
        "//cargo/vendor/hyper-0.11.9:hyper",
        "//cargo/vendor/hyper-tls-0.1.2:hyper_tls",
        "//cargo/vendor/libflate-0.1.12:libflate",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/mime_guess-2.0.0-alpha.3:mime_guess",
        "//cargo/vendor/native-tls-0.1.4:native_tls",
        "//cargo/vendor/serde-1.0.24:serde",
        "//cargo/vendor/serde_json-1.0.8:serde_json",
        "//cargo/vendor/serde_urlencoded-0.5.1:serde_urlencoded",
        "//cargo/vendor/tokio-core-0.1.10:tokio_core",
        "//cargo/vendor/tokio-io-0.1.4:tokio_io",
        "//cargo/vendor/tokio-tls-0.1.3:tokio_tls",
        "//cargo/vendor/url-1.6.0:url",
        "//cargo/vendor/uuid-0.5.1:uuid",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "simple" with type "example" omitted
# Unsupported target "timeouts" with type "test" omitted
