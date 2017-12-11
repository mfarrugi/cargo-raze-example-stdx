
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "example",
    srcs = ["example.rs"],
    deps = [
	"//cargo/vendor:log",
        "//cargo/vendor:env_logger",
        "//cargo/vendor:failure",
    ],
)


