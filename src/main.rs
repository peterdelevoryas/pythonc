#[macro_use]
extern crate serde_derive;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate pythonc;

use docopt::Docopt;
use pythonc::Result;
use std::path::Path;
use std::path::PathBuf;

const USAGE: &str = "
pythonc.

Usage:
    pythonc <source> [--runtime=<runtime>] [--out=<out>]
    pythonc (-h | --help)
    pythonc --version

Options:
    -h --help   Show this message.
    --version   Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_runtime: Option<String>,
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
        return Ok(())
    }

    let source = Path::new(&args.arg_source);
    let output = args.flag_out.map(PathBuf::from);

    if let Some(runtime) = args.flag_runtime {
        let asm = source.with_extension("s");
        pythonc::emit_asm(source, &asm)?;
        let output = output.unwrap_or(source.with_extension(""));
        pythonc::link(asm, runtime, output)?;
    } else {
        let output = output.unwrap_or(source.with_extension("s"));
        pythonc::emit_asm(source, output)?;
    }

    Ok(())
}
