//! Executable to generate huge cobertura XML files containing fake data
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// cobertura XML input file
    #[clap()]
    filename: PathBuf,
    /// minimum file size
    #[clap(short, long, default_value = "100000000")]
    min_size: usize,
}

const XML_HEADER: &str = r#"<?xml version="1.0" ?>
<!DOCTYPE coverage SYSTEM "https://cobertura.sourceforge.net/xml/coverage-04.dtd">
"#;

macro_rules! s {
    ( $e:expr ) => {
        $e.to_string().as_str()
    };
}

fn generate_cobertura_xml<P: AsRef<Path>>(filename: P, min_size: usize) -> anyhow::Result<()> {
    let mut writer = std::fs::File::create(filename)?;
    writer.write_all(XML_HEADER.as_bytes())?;
    let mut writer = Writer::new_with_indent(writer, b' ', 4);

    let mut elem = BytesStart::new("coverage");
    elem.push_attribute(("branch-rate", "1.3"));
    elem.push_attribute(("branches-covered", "23"));
    elem.push_attribute(("branches-valid", "12"));
    elem.push_attribute(("complexity", "0"));
    elem.push_attribute(("line-rate", "3.1"));
    elem.push_attribute(("lines-covered", "12"));
    elem.push_attribute(("lines-valid", "12"));
    elem.push_attribute(("timestamp", "1234567"));
    elem.push_attribute(("version", "2.0.3"));
    writer.write_event(Event::Start(elem))?;

    // Sources
    writer.write_event(Event::Start(BytesStart::new("sources")))?;

    writer.write_event(Event::Start(BytesStart::new("source")))?;
    writer.write_event(Event::Text(BytesText::new(".")))?;
    writer.write_event(Event::End(BytesEnd::new("source")))?;
    writer.write_event(Event::End(BytesEnd::new("sources")))?;

    // packages
    writer.write_event(Event::Start(BytesStart::new("packages")))?;
    let mut package_no = 0;
    loop {
        package_no += 1;
        if writer.get_ref().stream_position()? >= min_size as u64 {
            break;
        }
        let pkg_name = format!("package_{package_no}");
        let mut pkg = BytesStart::new("package");
        pkg.push_attribute(("line-rate", "1.1"));
        pkg.push_attribute(("branch-rate", "2.2"));
        pkg.push_attribute(("name", pkg_name.as_str()));
        pkg.push_attribute(("complexity", "0"));
        writer.write_event(Event::Start(pkg))?;
        // classes
        writer.write_event(Event::Start(BytesStart::new("classes")))?;

        for class_name in (0..10).map(|v| format!("class_{v}")) {
            let mut class = BytesStart::new("class");
            class.push_attribute(("branch-rate", "2.2"));
            class.push_attribute(("complexity", "0"));
            class.push_attribute(("filename", class_name.as_str()));
            class.push_attribute(("line-rate", "3.4"));
            class.push_attribute(("name", class_name.as_str()));
            writer.write_event(Event::Start(class))?;
            // methods
            writer.write_event(Event::Start(BytesStart::new("methods")))?;

            for (method_name, (line, hits)) in (0..100).map(|v| (format!("method_{v}"), (v, 1))) {
                let mut method = BytesStart::new("method");
                method.push_attribute(("name", method_name.as_str()));
                method.push_attribute(("signature", ""));
                method.push_attribute(("complexity", "0"));
                method.push_attribute(("line-rate", "0.4"));
                method.push_attribute(("branch-rate", "1.2"));
                writer.write_event(Event::Start(method))?;
                // Method lines (always exactly one?)
                writer.write_event(Event::Start(BytesStart::new("lines")))?;

                writer
                    .create_element("line")
                    .with_attributes([
                        ("hits", s!(hits)),
                        ("number", s!(line)),
                        ("branch", "false"),
                    ])
                    .write_empty()?;

                // close method lines
                writer.write_event(Event::End(BytesEnd::new("lines")))?;

                // close methods
                writer.write_event(Event::End(BytesEnd::new("method")))?;
            }
            writer.write_event(Event::End(BytesEnd::new("methods")))?;
            // add class lines
            writer.write_event(Event::Start(BytesStart::new("lines")))?;
            for line_number in 0..20 {
                let branch = true.to_string();
                let hits = 50.to_string();
                let number = line_number.to_string();
                let mut attrs = vec![
                    ("branch", branch.as_str()),
                    ("hits", hits.as_str()),
                    ("number", number.as_str()),
                ];
                let total = 100;
                let covered = 80;
                let percentage = covered * 100 / total;
                let cond_cov = format!("{percentage}% ({covered}/{total})");
                attrs.push(("condition-coverage", cond_cov.as_str()));
                writer
                    .create_element("line")
                    .with_attributes(attrs.into_iter())
                    .write_empty()?;

                // close class lines
            }
            writer.write_event(Event::End(BytesEnd::new("lines")))?;
            // close class
            writer.write_event(Event::End(BytesEnd::new("class")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("classes")))?;
        // close package
        writer.write_event(Event::End(BytesEnd::new("package")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("packages")))?;

    // close coverage
    writer.write_event(Event::End(BytesEnd::new("coverage")))?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    generate_cobertura_xml(&args.filename, args.min_size)?;
    Ok(())
}
