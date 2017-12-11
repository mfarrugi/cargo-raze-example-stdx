
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "http",
    srcs = ["http.rs"],
    deps = [
	"//cargo/vendor:reqwest",
    ],
)


