//! executable to run the conversion

use clap::Parser;
use std::io::BufRead;
use std::path::PathBuf;
use std::time::SystemTime;

use lcov2corbertura as lcov2xml;

/// Cmd line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// LCOV input files, when not given reads from stdin
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
        // merge into memory and pass to lineparser
        let mut report = lcov::Report::new();

        let mut totalsize = 0;
        for filename in args.files {
            totalsize += filename.metadata()?.len();
            report.merge(lcov::Report::from_file(filename)?)?;
        }

        let mut merged = String::with_capacity(totalsize as usize);
        for record in report.into_records() {
            merged.push_str(&record.to_string());
            merged.push('\n');
        }

        lcov2xml::parse_lines(
            merged.as_bytes().lines(),
            args.base_dir.as_path(),
            excludes.clone(),
        )?
    } else {
        let filename = args.files.get(0).unwrap().to_path_buf();
        lcov2xml::parse_fn(
            filename.as_path(),
            args.base_dir.as_path(),
            excludes.clone(),
        )?
    };

    // this is done repetitively to avoid dynamic dispatching. when a fourth demangler is added
    // implement enum dispatching ;)
    if args.demangle {
        if args.demangler == "$rust" {
            let demangler = lcov2xml::RustDemangler::new();
            lcov2xml::coverage_to_file(&args.output, &result, now(), demangler)?;
        } else {
            let demangler = lcov2xml::CppDemangler::new(&args.demangler)?;
            lcov2xml::coverage_to_file(&args.output, &result, now(), demangler)?;
        }
    } else {
        let demangler = lcov2xml::NullDemangler::new();
        lcov2xml::coverage_to_file(&args.output, &result, now(), demangler)?;
    };
    Ok(())
}
