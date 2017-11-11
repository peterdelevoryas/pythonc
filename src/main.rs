#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
extern crate pythonc;

use pythonc::error::*;
use std::path::PathBuf;

/*
docopt!(Args derive Debug, "
pythonc.

Usage:
    pythonc [options] INPUT
    pythonc (-h | -v)

Options:
    --emit STAGE    Configure output stage.
                    [values: ast, explicated, flattened, vasm, liveness, asm, obj, bin]
                    [default: asm]
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
*/

quick_main!(run);

fn run() -> pythonc::Result<()> {
    let matches = clap::App::new("pythonc")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!("\n"))
        .get_matches();

    let pythonc = pythonc::Pythonc::new();

    /*
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
    */
    unimplemented!()
}
