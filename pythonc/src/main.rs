#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate pythonc;

use pythonc::error::*;
use std::path::PathBuf;

docopt!(Args derive Debug, "
pythonc.

Usage:
    pythonc [options] INPUT
    pythonc (-h | -v)

Options:
    --emit STAGE    Configure output stage.
                    [values: pystr, pyast, ast, ir, vasm, liveness, asm, obj]
                    [default: asm]
                    pystr is the str() output of the official Python parser.
                    pyast is how pythonc parses pystr.
                    ast is the conversion from pyast to pythonc's ast.
    --runtime LIB   Path to runtime.
    -o PATH         Configures output path.
    --stdout        Print output to stdout (instead of to file).
    -h --help       Show this message.
    -v --version    Show version.
",
    arg_INPUT: PathBuf,
    flag_emit: pythonc::Stage,
    flag_runtime: Option<PathBuf>,
    flag_o: Option<PathBuf>,
    flag_stdout: bool,
);

quick_main!(run);

fn run() -> pythonc::Result<()> {
    let args: Args = Args::docopt().deserialize().unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let pythonc = pythonc::Pythonc::new();

    let in_path = &args.arg_INPUT;
    let out_path = if args.flag_stdout {
        Some(PathBuf::from("/dev/stdout"))
    } else {
        args.flag_o
    };
    let runtime = args.flag_runtime;
    let stop_stage = args.flag_emit;

    if stop_stage == pythonc::Stage::Bin {
        if runtime.is_none() {
            return Err("Cannot emit binary without runtime".into());
        }
    }

    pythonc
        .emit(in_path, stop_stage, out_path, runtime)
        .chain_err(|| format!("Could not compile {:?}", in_path.display()))
}
