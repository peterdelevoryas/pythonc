extern crate pythonc_token as token;
extern crate pythonc_ast as ast;
extern crate pythonc_trans as trans;
#[macro_use]
extern crate error_chain;

pub mod error;
pub use error::{Error, ErrorKind, Result, ResultExt};

use std::path::Path;
use std::process::Command;

pub fn compile(source: &str) -> Result<String> {
    let tokens = token::Stream::new(source);
    let ast = ast::parse_program(tokens).chain_err(|| "parse error")?;
    let ir = ast.into();
    let asm = trans::Builder::build(&ir);
    Ok(asm)
}

pub fn emit_asm<P1, P2>(source: P1, output: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let source = read_file(source).chain_err(|| "reading source file")?;
    let asm = compile(&source).chain_err(
        || format!("compiling source file {:?}", source)
    )?;

    write_file(&asm, output)
}

pub fn link<P1, P2, P3>(asm: P1, runtime: P2, output: P3) -> Result<()>
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

pub fn read_file<P>(path: P) -> Result<String>
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

pub fn write_file<P>(data: &str, path: P) -> Result<()>
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
