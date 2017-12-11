# Print out raze targets that fail to build.
bazel query cargo/vendor/... | xargs -l -I % bash -c "bazel build % > /dev/null 2>&1 || echo %"
