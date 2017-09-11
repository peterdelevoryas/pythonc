#[macro_use]
extern crate serde_derive;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate pythonc;

use docopt::Docopt;
use pythonc::Result;
use pythonc::Compiler;
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
        return Ok(());
    }

    let mut compiler = Compiler::new(&args.arg_source);

    if let Some(runtime) = args.flag_runtime {
        compiler.runtime(runtime);
    }

    if let Some(out) = args.flag_out {
        compiler.out_path(out);
    }

    compiler.run()
}
