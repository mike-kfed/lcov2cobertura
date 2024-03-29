//! executable to run the conversion

use clap::Parser;
use std::io::{BufRead, Read};
use std::path::PathBuf;
use std::time::SystemTime;

use lcov2cobertura as lcov2xml;

/// Cmd line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// LCOV input files, use single dash '-' argument to read from stdin
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
    /// splits XML file into 9.5MB big chunks for GitLab, attention keeps original file intact
    #[clap(long)]
    split_xml: bool,
}

fn now() -> anyhow::Result<u64> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => Ok(n.as_secs()),
        Err(_) => anyhow::bail!("SystemTime before UNIX EPOCH!"),
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
            &excludes,
        )?
    } else if args.files.first() == Some(&PathBuf::from("-")) {
        let mut input = Vec::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_end(&mut input)?;
        lcov2xml::parse_lines(input.lines(), args.base_dir.as_path(), &excludes)?
    } else {
        let filename = args
            .files
            .first()
            .ok_or_else(|| anyhow::anyhow!("no filename given"))?
            .to_path_buf();
        lcov2xml::parse_file(filename.as_path(), args.base_dir.as_path(), &excludes)?
    };

    // this is done repetitively to avoid dynamic dispatching. when a fourth demangler is added
    // implement enum dispatching ;)
    if args.demangle {
        if args.demangler == "$rust" {
            let demangler = lcov2xml::RustDemangler::new();
            lcov2xml::coverage_to_file(&args.output, &result, now()?, demangler)?;
        } else {
            let demangler = lcov2xml::CppDemangler::new(&args.demangler)?;
            lcov2xml::coverage_to_file(&args.output, &result, now()?, demangler)?;
        }
    } else {
        let demangler = lcov2xml::NullDemangler::new();
        lcov2xml::coverage_to_file(&args.output, &result, now()?, demangler)?;
    };

    if args.split_xml {
        lcov2xml::corbertura_xml_split(&args.output)?;
    }
    Ok(())
}
