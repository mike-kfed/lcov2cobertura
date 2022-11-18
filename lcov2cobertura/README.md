# lcov2cobertura

converts lcov info files to cobertura XML

Idea is for this to be a library for [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) but also a more performant standalone application replacing the Python based [lcov-to-cobertura-xml](https://github.com/eriwen/lcov-to-cobertura-xml)

## Features

- can demangle C++ names
- can demangle rustc names
- merges multiple lcov reports into one
- optionally writes many cobertura XML files
