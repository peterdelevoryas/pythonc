extern crate python_token as token;
extern crate python_ast as ast;
extern crate python_trans as trans;
#[macro_use]
extern crate error_chain;

pub mod error;
pub use error::{Error, ErrorKind, Result, ResultExt};

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub struct Compiler {
    source: PathBuf,
    runtime: Option<PathBuf>,
    out_path: Option<PathBuf>,
    create_new: bool,
}

impl Compiler {
    pub fn new<P>(source: P) -> Compiler
    where
        P: Into<PathBuf>,
    {
        Compiler {
            source: source.into(),
            runtime: None,
            out_path: None,
            create_new: true,
        }
    }

    pub fn runtime<P>(&mut self, path: P) -> &mut Compiler
    where
        P: Into<PathBuf>,
    {
        self.runtime = Some(path.into());
        self
    }

    pub fn out_path<P>(&mut self, path: P) -> &mut Compiler
    where
        P: Into<PathBuf>,
    {
        self.out_path = Some(path.into());
        self
    }

    pub fn create_new(&mut self, create_new: bool) -> &mut Compiler {
        self.create_new = create_new;
        self
    }

    pub fn run(&self) -> Result<()> {
        if let Some(ref runtime) = self.runtime {
            let asm = self.source.with_extension("s");
            emit_asm(&self.source, &asm, self.create_new)?;
            let out_path = self.out_path.clone().unwrap_or(
                self.source.with_extension(""),
            );
            link(asm, runtime, out_path)?;
        } else {
            let out_path = self.out_path.clone().unwrap_or(
                self.source.with_extension("s"),
            );
            emit_asm(&self.source, out_path, self.create_new)?;
        }
        Ok(())
    }
}

pub fn compile(source: &str) -> Result<trans::Program> {
    let tokens = token::Stream::new(source);
    let ast = ast::parse_program(tokens).chain_err(|| "parse error")?;
    let ir = ast.into();
    let asm = trans::Program::build(&ir);
    Ok(asm)
}

pub fn emit_asm<P1, P2>(source: P1, output: P2, create_new: bool) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let source = read_file(source).chain_err(|| "reading source file")?;
    let asm = compile(&source).chain_err(|| {
        format!("compiling source file {:?}", source)
    })?;

    write_file(trans::Att(&asm), output, create_new)
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
        .and_then(|e| if !e.success() {
            Err(ErrorKind::Link(e).into())
        } else {
            Ok(())
        })
}

fn read_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    use std::fs::File;
    use std::io::Read;

    let path = path.as_ref();
    let mut f = File::open(path).chain_err(|| {
        format!("opening file {:?}", path.display())
    })?;
    let size = f.metadata().chain_err(|| "getting file size")?.len() as usize;
    let mut s = String::with_capacity(size);
    f.read_to_string(&mut s).chain_err(|| "reading file")?;
    Ok(s)
}

fn write_file<P, D>(data: D, path: P, create_new: bool) -> Result<()>
where
    P: AsRef<Path>,
    D: ::std::fmt::Display,
{
    use std::fs::OpenOptions;
    use std::io::Write;

    let path = path.as_ref();
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(create_new)
        .open(path)
        .chain_err(|| format!("creating file {:?}", path.display()))?;
    write!(f, "{}", data).chain_err(|| "writing data")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
