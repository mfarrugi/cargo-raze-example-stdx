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


# Unsupported target "array" with type "test" omitted
# Unsupported target "array-construct" with type "test" omitted
# Unsupported target "axis_ops" with type "example" omitted
# Unsupported target "azip" with type "test" omitted
# Unsupported target "bench1" with type "bench" omitted
# Unsupported target "bounds_check_elim" with type "example" omitted
# Unsupported target "broadcast" with type "test" omitted
# Unsupported target "chunks" with type "bench" omitted
# Unsupported target "clone" with type "test" omitted
# Unsupported target "column_standardize" with type "example" omitted
# Unsupported target "complex" with type "test" omitted
# Unsupported target "construct" with type "bench" omitted
# Unsupported target "convo" with type "example" omitted
# Unsupported target "dimension" with type "test" omitted
# Unsupported target "format" with type "test" omitted
# Unsupported target "gemv" with type "bench" omitted
# Unsupported target "higher-order" with type "bench" omitted
# Unsupported target "indices" with type "test" omitted
# Unsupported target "into-ixdyn" with type "test" omitted
# Unsupported target "iter" with type "bench" omitted
# Unsupported target "iterator_chunks" with type "test" omitted
# Unsupported target "iterators" with type "test" omitted
# Unsupported target "ix0" with type "test" omitted
# Unsupported target "ixdyn" with type "test" omitted
# Unsupported target "life" with type "example" omitted
rust_library(
    name = "ndarray",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/itertools-0.7.4:itertools",
        "//cargo/vendor/matrixmultiply-0.1.14:matrixmultiply",
        "//cargo/vendor/num-complex-0.1.41:num_complex",
        "//cargo/vendor/num-traits-0.1.41:num_traits",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "numeric" with type "bench" omitted
# Unsupported target "oper" with type "test" omitted
# Unsupported target "rollaxis" with type "example" omitted
# Unsupported target "s" with type "test" omitted
# Unsupported target "sort-axis" with type "example" omitted
# Unsupported target "stacking" with type "test" omitted
# Unsupported target "windows" with type "test" omitted
# Unsupported target "zip_many" with type "example" omitted
# Unsupported target "zst" with type "test" omitted
