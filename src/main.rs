extern crate rust_python;
extern crate error_chain;

use rust_python::Result;
use error_chain::ChainedError;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn run() -> Result<()> {
    let path = env::args().nth(1).ok_or("not enough arguments")?;
    let source = {
        let mut f = fs::File::open(&path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        buf
    };
    let source = source.as_str();
    let compiler = rust_python::Compiler::new();
    let asm = compiler.compile(source)?;
    let out_path = Path::new(&path).with_extension("s");
    let mut f = fs::File::create(&out_path)?;
    f.write_all(asm.as_bytes())?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e.display_chain());
    }
}
