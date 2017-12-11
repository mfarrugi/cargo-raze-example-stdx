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
    name = "mime_guess_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "src/gen_mime_types.rs",
    data = glob([
      "*"
    ]),
    deps = [
      "//cargo/vendor/phf_codegen-0.7.21:phf_codegen",
      "//cargo/vendor/unicase-1.4.2:unicase",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    visibility = ["//visibility:private"],
)

genrule(
    name = "mime_guess_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["mime_guess_out_dir_outputs.tar.gz"],
    tools = [":mime_guess_build_script"],
    local = 1,
    cmd = "mkdir mime_guess_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/cargo/vendor/mime_guess-2.0.0-alpha.3\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        
        + " export OUT_DIR=$$PWD/mime_guess_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :mime_guess_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
rust_library(
    name = "mime_guess",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/mime-0.3.5:mime",
        "//cargo/vendor/phf-0.7.21:phf",
        "//cargo/vendor/unicase-1.4.2:unicase",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    out_dir_tar = ":mime_guess_build_script_executor",
    crate_features = [
        
    ],
)
# Unsupported target "rev_map" with type "example" omitted
