#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate pythonc;

use docopt::Docopt;

const USAGE: &str = "
pythonc.

Usage:
    pythonc --emit-asm <file>
    pythonc (-h | --help)
    pythonc --version

Options:
    -h --help   Show this message.
    --version   Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_emit_asm: bool,
    flag_version: bool,
    arg_file: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
    println!("{:?}", args);
}
