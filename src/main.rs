#[macro_use]
extern crate serde_derive;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate pythonc;

use docopt::Docopt;
use pythonc::{Result, ResultExt};
use std::path::Path;
use std::path::PathBuf;

const USAGE: &str = "
pythonc.

Usage:
    pythonc --emit-asm <source> [--out=<out>]
    pythonc (-h | --help)
    pythonc --version

Options:
    -h --help   Show this message.
    --version   Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_emit_asm: bool,
    flag_out: Option<String>,
    flag_version: bool,
    arg_source: String,
}

quick_main!(run);

fn run() -> Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    } else if args.flag_emit_asm {
        let source = Path::new(&args.arg_source);
        let output = args.flag_out.map(PathBuf::from)
            .unwrap_or(source.with_extension("s"));
        emit_asm(source, output)?;
    }
    Ok(())
}

fn emit_asm<P1, P2>(source: P1, output: P2) -> Result<()>
    where P1: AsRef<Path>,
          P2: AsRef<Path>,
{

    let source = read_file(source).chain_err(|| "reading source file")?;
    let asm = pythonc::compile(&source).chain_err(|| "compiling source file")?;

    write_file(&asm, output)
}

fn read_file<P>(path: P) -> Result<String>
    where P: AsRef<Path>
{
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(path).chain_err(|| "opening file")?;
    let size = f.metadata().chain_err(|| "getting file size")?.len() as usize;
    let mut s = String::with_capacity(size);
    f.read_to_string(&mut s).chain_err(|| "reading file")?;
    Ok(s)
}

fn write_file<P>(data: &str, path: P) -> Result<()>
    where P: AsRef<Path>
{
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .chain_err(|| "creating file")?;
    f.write_all(data.as_bytes()).chain_err(|| "writing data")?;
    Ok(())
}
