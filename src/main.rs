#[macro_use]
extern crate serde_derive;
extern crate docopt;
#[macro_use]
extern crate error_chain;
extern crate pythonc;

use docopt::Docopt;
use pythonc::{ErrorKind, Result, ResultExt};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

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
        emit_asm(source, &asm)?;
        let output = output.unwrap_or(source.with_extension(""));
        link(asm, runtime, output)?;
    } else {
        let output = output.unwrap_or(source.with_extension("s"));
        emit_asm(source, output)?;
    }

    Ok(())
}

fn emit_asm<P1, P2>(source: P1, output: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let source = read_file(source).chain_err(|| "reading source file")?;
    let asm = pythonc::compile(&source).chain_err(
        || format!("compiling source file {:?}", source)
    )?;

    write_file(&asm, output)
}

fn link<P1, P2, P3>(asm: P1, runtime: P2, output: P3) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    let (asm, runtime, output) = (asm.as_ref(), runtime.as_ref(), output.as_ref());
    Command::new("gcc")
        .args(&["-m32", "-g"])
        .args(&[asm.as_os_str(), runtime.as_os_str()])
        .arg("-o")
        .arg(output.as_os_str())
        .spawn()
        .chain_err(|| "spawning gcc")?
        .wait()
        .chain_err(|| "gcc wasn't running")
        .and_then(|e| {
            if !e.success() { 
                Err(ErrorKind::Link(e).into())
            } else {
                Ok(())
            }
        })
}

fn read_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(path.as_ref()).chain_err(|| {
        format!("opening file {:?}", path.as_ref().to_string_lossy())
    })?;
    let size = f.metadata().chain_err(|| "getting file size")?.len() as usize;
    let mut s = String::with_capacity(size);
    f.read_to_string(&mut s).chain_err(|| "reading file")?;
    Ok(s)
}

fn write_file<P>(data: &str, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.as_ref())
        .chain_err(|| {
            format!("creating file {:?}", path.as_ref().to_string_lossy())
        })?;
    f.write_all(data.as_bytes()).chain_err(|| "writing data")?;
    Ok(())
}
