# Use the latest version of rust instead of whatever is in the rules_rust repository.

load("@io_bazel_rules_rust//rust:repositories.bzl", 
     "RUST_DARWIN_BUILD_FILE", 
     "RUST_LINUX_BUILD_FILE"
     )

TOOLCHAINS = """
load("@io_bazel_rules_rust//rust:toolchain.bzl", "rust_toolchain")

toolchain(
    name = "rust-linux-x86_64",
    exec_compatible_with = [
        "@bazel_tools//platforms:linux",
        "@bazel_tools//platforms:x86_64",
    ],
    target_compatible_with = [
        "@bazel_tools//platforms:linux",
        "@bazel_tools//platforms:x86_64",
    ],
    toolchain = ":rust-linux-x86_64_impl",
    toolchain_type = "@io_bazel_rules_rust//rust:toolchain",
)
rust_toolchain(
    name = "rust-linux-x86_64_impl",
    rust_doc = "@rust_linux_x86_64//:rustdoc",
    rust_lib = ["@rust_linux_x86_64//:rust_lib"],
    rustc = "@rust_linux_x86_64//:rustc",
    rustc_lib = ["@rust_linux_x86_64//:rustc_lib"],
    visibility = ["//visibility:public"],
)

toolchain(
    name = "rust-nightly-linux-x86_64",
    exec_compatible_with = [
        "@bazel_tools//platforms:linux",
        "@bazel_tools//platforms:x86_64",
    ],
    target_compatible_with = [
        "@bazel_tools//platforms:linux",
        "@bazel_tools//platforms:x86_64",
    ],
    toolchain = ":rust-nightly-linux-x86_64_impl",
    toolchain_type = "@io_bazel_rules_rust//rust:toolchain",
)
rust_toolchain(
    name = "rust-nightly-linux-x86_64_impl",
    rust_doc = "@rust_nightly_linux_x86_64//:rustdoc",
    rust_lib = ["@rust_nightly_linux_x86_64//:rust_lib"],
    rustc = "@rust_nightly_linux_x86_64//:rustc",
    rustc_lib = ["@rust_nightly_linux_x86_64//:rustc_lib"],
    visibility = ["//visibility:public"],
)

toolchain(
    name = "rust-darwin-x86_64",
    exec_compatible_with = [
        "@bazel_tools//platforms:osx",
        "@bazel_tools//platforms:x86_64",
    ],
    target_compatible_with = [
        "@bazel_tools//platforms:osx",
        "@bazel_tools//platforms:x86_64",
    ],
    toolchain = ":rust-darwin-x86_64_impl",
    toolchain_type = "@io_bazel_rules_rust//rust:toolchain",
)
rust_toolchain(
    name = "rust-darwin-x86_64_impl",
    rust_doc = "@rust_darwin_x86_64//:rustdoc",
    rust_lib = ["@rust_darwin_x86_64//:rust_lib"],
    rustc = "@rust_darwin_x86_64//:rustc",
    rustc_lib = ["@rust_darwin_x86_64//:rustc_lib"],
    visibility = ["//visibility:public"],
)
"""


def rust_toolchain_repositories():
  native.new_http_archive(
      name = "rust_linux_x86_64",
      url = "https://static.rust-lang.org/dist/rust-1.22.0-x86_64-unknown-linux-gnu.tar.gz",
      strip_prefix = "rust-1.22.0-x86_64-unknown-linux-gnu",
      sha256 = "11118f670343f3ebdd4790f845fd68f38db65b19261b81b3ab580d8425d0a7c6",
      build_file_content = RUST_LINUX_BUILD_FILE,
  )

  native.new_http_archive(
      name = "rust_nightly_linux_x86_64",
      url = "https://static.rust-lang.org/dist/2017-12-10/rust-nightly-x86_64-unknown-linux-gnu.tar.gz",
      strip_prefix = "rust-nightly-x86_64-unknown-linux-gnu",
      sha256 = "bd042beec19eae1872bd133a3d12c5007830350ad3b94f2802ebc75b58d98201",
      build_file_content = RUST_LINUX_BUILD_FILE,
  )

  native.new_http_archive(
      name = "rust_darwin_x86_64",
      url = "https://static.rust-lang.org/dist/rust-1.22.1-x86_64-apple-darwin.tar.gz",
      strip_prefix = "rust-1.22.1-x86_64-apple-darwin",
      sha256 = "c7cf38a9fe56cc03b61213899e0e2db2153ce4c69ed36b794264c5d3629dae57",
      build_file_content = RUST_DARWIN_BUILD_FILE,
  )

  native.new_local_repository(
      name = "rust_default_toolchains",
      path = ".",
      build_file_content = TOOLCHAINS)

  # nb. Ordering matters, bazel defaults to first matching toolchain.
  # Pass '--extra_toolchains @rust_default_toolchains//:rust-nightly-linux-x86_64' to bazel to use nightly instead of stable.
  native.register_toolchains(
      "@rust_default_toolchains//:rust-linux-x86_64",
      "@rust_default_toolchains//:rust-nightly-linux-x86_64",
      "@rust_default_toolchains//:rust-darwin-x86_64")
