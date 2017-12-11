git_repository(
    name = "io_bazel_rules_rust",
    commit = "0d0693267ba3d338230bae204bbdbfd20da0366e",
    remote = "https://github.com/mfarrugi/rules_rust.git",
)

load("//bazel:custom_rust_toolchain.bzl", "rust_toolchain_repositories")
rust_toolchain_repositories()


# Make some system libraries available, to provide dependencies to sys crates. 
new_local_repository(
    name = "linux_usr_lib",
    path = "/usr/lib/x86_64-linux-gnu",
    build_file_content =
"""
package(default_visibility = ["//visibility:public"])

cc_library(
    name = "z",
    srcs = ["libz.a"],
)

cc_library(
    name = "openssl",
    srcs = [
    	"libssl.so",
	"libcrypto.so",
    ],
)
""",
)
