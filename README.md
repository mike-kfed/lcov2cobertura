# lcov2cobertura

converts lcov info files to cobertura XML

Idea is for this to be a library for [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) but also a more performant standalone application replacing the Python based [lcov-to-cobertura-xml](https://github.com/eriwen/lcov-to-cobertura-xml)

## Features

- can demangle C++ names
- can demangle rustc names
- merges multiple lcov reports into one
- can split big XML files into many smaller ones for GitLab attachment size limitation. strategy: it generates 9.5MB big xml files, fitting as many packages as possible into each file

## Usage

```bash
lcov2xml --help
# this would write file coverage.xml
lcov2xml lcov.info
# this splits an existing xml file into smaller ones
cobertura_split coverage.xml
```

### available cmd-line args

```
convert LCOV info file to cobertura XML format

Usage: lcov2xml [OPTIONS] [FILES]...

Arguments:
  [FILES]...  LCOV input files, when not given reads from stdin

Options:
  -b, --base-dir <BASE_DIR>    Directory where source files are located [default: .]
  -o, --output <OUTPUT>        Path to store cobertura xml file [default: coverage.xml]
  -e, --excludes <EXCLUDES>    Comma-separated list of regexes of packages to exclude [default: ]
  -d, --demangle               Demangle function names
      --demangler <DEMANGLER>  Path to demangler tool, e.g. c++filt for C++, $rust = internal rustc demangler [default: $rust]
      --split-xml              splits XML file into 9.5MB big chunks for GitLab, attention keeps original file intact
  -h, --help                   Print help information
  -V, --version                Print version information
```

## Performance

Ran on a about 500KiB sized lcov.info file on macOS and measured the wall-clock time plus max RAM usage.
Does not seem to be much faster in gross runtime but uses an order of magnitude less RAM.

For the coverage.xml splitting tool input is a 100MB sized xml file. RAM usage is drastically reduced.

```bash
/usr/bin/time -al python3 lcov_cobertura.py lcov.info
/usr/bin/time -al cargo run --release --bin lcov2xml -- lcov.info

/usr/bin/time -al python3 split-by-package-int.py huge.xml outdir
/usr/bin/time -al cargo run --release --bin cobertura_split -- huge.xml
```

|            | Python 3.10                  | Rust 1.65              |
| ---------- | ---------------------------- | ---------------------- |
| what       | lcov-to-cobertura-xml v2.0.2 | lcov2cobertura v1.0.0  |
| runtime    | 0.38secs                     | 0.32sec                |
| memory     | 64MiB                        | 3MiB                   |
| _splitter_ |                              |                        |
| what       | split-by-package-int         | cobertura_split v1.0.2 |
| runtime    | 2.38secs                     | 0.32sec                |
| memory     | 2GiB                         | 13MiB                  |
