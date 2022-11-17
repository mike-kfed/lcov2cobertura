//! Interface and implemtation of different demanglers
use regex::Regex;
use rustc_demangle::demangle;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub trait Demangler {
    fn demangle(&mut self, ident: &str) -> io::Result<String>;
    fn stop(self) -> io::Result<()>;
}

pub struct CppDemangler {
    child: Child,
    child_in: ChildStdin,
    child_out: BufReader<ChildStdout>,
}

impl CppDemangler {
    pub fn new(cmd: &str) -> io::Result<Self> {
        let mut child = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let child_in = child.stdin.take().unwrap();
        let child_out = BufReader::new(child.stdout.take().unwrap());
        Ok(Self {
            child,
            child_in,
            child_out,
        })
    }
}

impl Demangler for CppDemangler {
    fn demangle(&mut self, ident: &str) -> io::Result<String> {
        self.child_in.write_all(format!("{}\n", ident).as_bytes())?;
        let mut line = String::new();
        self.child_out.read_line(&mut line)?;
        Ok(line.trim().to_string())
    }

    fn stop(mut self) -> io::Result<()> {
        self.child.kill()?;
        Ok(())
    }
}

pub struct RustDemangler {
    /// strips crate disambiguators
    disambiguator: Regex,
}
impl RustDemangler {
    pub fn new() -> Self {
        Self {
            disambiguator: Regex::new(r"\[[0-9a-f]{5,16}\]::").unwrap(),
        }
    }
}
impl Demangler for RustDemangler {
    fn demangle(&mut self, ident: &str) -> io::Result<String> {
        let demangled = demangle(ident).to_string();
        Ok(self.disambiguator.replace_all(&demangled, "::").to_string())
    }

    fn stop(self) -> io::Result<()> {
        Ok(())
    }
}

pub struct NullDemangler {}
impl NullDemangler {
    pub fn new() -> Self {
        Self {}
    }
}
impl Demangler for NullDemangler {
    fn demangle(&mut self, ident: &str) -> io::Result<String> {
        Ok(ident.to_string())
    }

    fn stop(self) -> io::Result<()> {
        Ok(())
    }
}
