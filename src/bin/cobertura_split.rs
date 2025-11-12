//! executable to efficiently split huge cobertura XML files
use std::path::PathBuf;

use clap::Parser;

use lcov2cobertura as lcov2xml;

/// Cmd line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about = "Split cobertura XML into 9.5MB chunks", long_about = None)]
struct Args {
    /// cobertura XML input file
    #[clap()]
    filename: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    lcov2xml::corbertura_xml_split(args.filename)?;
    Ok(())
}
