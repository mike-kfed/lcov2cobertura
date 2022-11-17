//! executable to run the conversion

use clap::Parser;
use std::io::BufRead;
use std::path::PathBuf;
use std::time::SystemTime;

use lcov2corbertura as lcov;

/// Cmd line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Lcov input files, when not given reads from stdin
    #[clap()]
    files: Vec<PathBuf>,
    /// Directory where source files are located
    #[clap(short, long, default_value = ".")]
    base_dir: PathBuf,
    /// Path to store cobertura xml file
    #[clap(short, long, default_value = "coverage.xml")]
    output: PathBuf,
    /// Comma-separated list of regexes of packages to exclude
    #[clap(short, long, default_value = "")]
    excludes: String,
    /// Demangle function names
    #[clap(short, long)]
    demangle: bool,
    /// Path to demangler tool, e.g. c++filt for C++, $rust = internal rustc demangler
    #[clap(long, default_value = "$rust")]
    demangler: String,
}

fn now() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let excludes: Vec<&str> = args.excludes.split(',').filter(|v| !v.is_empty()).collect();
    let result = if args.files.len() > 1 {
        // TODO implement lcov file merge
        // merge into memory and pass to lineparser
        lcov::parse_lines(
            "".as_bytes().lines(),
            args.base_dir.as_path(),
            excludes.clone(),
        )?
    } else {
        let filename = args.files.get(0).unwrap().to_path_buf();
        lcov::parse_fn(
            filename.as_path(),
            args.base_dir.as_path(),
            excludes.clone(),
        )?
    };
    /*
    let demangler = lcov::NullDemangler::new();
    let lcov_xml = lcov::coverage_as_string(&result, 1346815648000, demangler).unwrap();
    println!("{}", lcov_xml);
    */

    // this is done repetitively to avoid dynamic dispatching. when a fourth demangler is added
    // implement enum dispatching ;)
    if args.demangle {
        if args.demangler == "$rust" {
            let demangler = lcov::RustDemangler::new();
            lcov::coverage_to_file(&args.output, &result, now(), demangler)?;
        } else {
            let demangler = lcov::CppDemangler::new(&args.demangler)?;
            lcov::coverage_to_file(&args.output, &result, now(), demangler)?;
        }
    } else {
        let demangler = lcov::NullDemangler::new();
        lcov::coverage_to_file(&args.output, &result, now(), demangler)?;
    };
    Ok(())
}
