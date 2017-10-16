#![feature(plugin, box_syntax)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate python;
extern crate python_ir;

use python::ResultExt;
use python::ErrorKind;
use std::path::PathBuf;
use std::path::Path;
use std::fmt;

docopt!(Args derive Debug, "
pythonc.

Usage:
    pythonc [options] INPUT
    pythonc (-h | -v)

Options:
    --emit STAGE    Configure output stage.
                    [values: pystr, pyast, ast, ir, vm, asm, obj]
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
    flag_emit: python::CompilerStage,
    flag_runtime: Option<PathBuf>,
    flag_o: Option<PathBuf>,
    flag_stdout: bool,
);

quick_main!(run);

//
// obj = gcc -m32 -g -c -o obj_path -xassembler -
// bin = gcc -m32 -g obj_path runtime_path -o bin_path
//
fn run() -> python::Result<()> {
    let args: Args = Args::docopt().deserialize().unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let compiler = python::Compiler::new();

    let input = &args.arg_INPUT;
    let out_path = if args.flag_stdout {
        Some(PathBuf::from("/dev/stdout"))
    } else {
        args.flag_o
    };
    compiler.emit(input, args.flag_emit, out_path, args.flag_runtime).chain_err(
        || {
            format!("Could not compile {:?}", input.display())
        },
    )
}
