# lcov2corbertura

converts lcov info files to corbertura XML

Idea is for this to be a library for [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) but also a more performant standalone application replacing the Python based [lcov-to-cobertura-xml](https://github.com/eriwen/lcov-to-cobertura-xml)

## Features

- can demangle C++ names
- can demangle rustc names
- TODO: merges multiple lcov reports into one

## Usage

```bash
lcov2xml --help
# this would write file coverage.xml
lcov2xml lcov.info
```

## Performance

Ran on a about 500KiB sized lcov.info file on macOS and measured the wall-clock time plus max RAM usage.
Does not seem to be much faster in gross runtime but uses an order of magnitude less RAM.

```bash
time python3 lcov_cobertura.py lcov.info
time cargo run --release -- lcov.info
```

|         | Python 3.10                  | Rust 1.65              |
| ------- | ---------------------------- | ---------------------- |
| what    | lcov-to-cobertura-xml v2.0.2 | lcov2corbertura v1.0.0 |
| runtime | 0.38secs                     | 0.32sec                |
| memory  | 64MiB                        | 3MiB                   |
