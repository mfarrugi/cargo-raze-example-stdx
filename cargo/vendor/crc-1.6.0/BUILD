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

rust_binary(
    name = "crc_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    data = glob([
      "*"
    ]),
    deps = [
      "//cargo/vendor/build_const-0.2.0:build_const",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
      "default",
      "std",
    ],
    visibility = ["//visibility:private"],
)

genrule(
    name = "crc_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["crc_out_dir_outputs.tar.gz"],
    tools = [":crc_build_script"],
    local = 1,
    cmd = "mkdir crc_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/cargo/vendor/crc-1.6.0\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_STD=1;"
        + " export OUT_DIR=$$PWD/crc_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :crc_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
# Unsupported target "bench" with type "bench" omitted
rust_library(
    name = "crc",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/build_const-0.2.0:build_const",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    out_dir_tar = ":crc_build_script_executor",
    crate_features = [
        "default",
        "std",
    ],
)
# Unsupported target "crc" with type "test" omitted
# Unsupported target "hash" with type "test" omitted
