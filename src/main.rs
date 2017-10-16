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
                    Valid values: py_repr, ast, ir, vm, asm, obj.
                    [default: asm]
    --runtime LIB   Path to runtime.
    -o PATH         Configures output path.
    -h --help       Show this message.
    -v --version    Show version.
",
    arg_INPUT: PathBuf,
    flag_emit: python::CompilerStage,
    flag_runtime: Option<PathBuf>,
    flag_o: Option<PathBuf>,
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

    let compiler = match args.flag_runtime {
        Some(runtime) => python::Compiler::with_runtime(runtime),
        None => python::Compiler::new(),
    };

    let input = &args.arg_INPUT;
    let out_path = args.flag_o.as_ref().map(|pathbuf| pathbuf.as_ref());
    compiler.emit(input, args.flag_emit, out_path).chain_err(
        || {
            format!("Could not compile {:?}", input.display())
        },
    )
}
