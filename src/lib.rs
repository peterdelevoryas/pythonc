#![feature(box_syntax, box_patterns, conservative_impl_trait)]
#[macro_use] extern crate error_chain;
#[macro_use] extern crate util;
#[macro_use] extern crate clap;
extern crate slab;
extern crate ast;

pub mod error;
pub mod explicate;
pub mod heapify;
pub mod raise;
pub mod flatten;

use flatten::Flatten;

pub use error::*;

use std::path::Path;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug)]
pub struct Pythonc {}

arg_enum!{
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Stage {
        Ast,
        Explicated,
        Heapified,
        Raised,
        Flattened,
        VAsm,
        Liveness,
        Asm,
        Obj,
        Bin
    }
}

impl Pythonc {
    pub fn new() -> Pythonc {
        Pythonc {}
    }

    pub fn emit(
        &self,
        in_path: &Path,
        stop_stage: Stage,
        out_path: Option<PathBuf>,
        runtime: Option<PathBuf>,
        show_casts: bool,
    ) -> Result<()> {
        let out_path = out_path.unwrap_or(in_path.with_extension(stop_stage.file_ext()));
        let out_path = &out_path;
        let source = read_file(in_path).chain_err(|| {
            format!("Unable to read input file {:?}", in_path.display())
        })?;

        let parser = ast::Parser::new();
        let ast = parser.parse(&source).chain_err(|| "Unable to parse source")?;
        if stop_stage == Stage::Ast {
            let ast = format!("{:#?}", ast);
            return write_out(ast, out_path);
        }

        let mut explicate = explicate::Explicate::new();
        let explicated = explicate.module(ast);
        {
            use explicate::TypeCheck;
            // type check!
            let result = {
                let mut type_env = explicate::TypeEnv::new(&explicate);
                explicated.type_check(&mut type_env)
            };
            result.chain_err(|| {
                // always show casts during type checking output
                let fmt = explicate::Formatter::new(&explicate, &explicated, true);
                format!("{}", fmt)
            }).chain_err(|| "Error type checking explicated ast")?;
        }
        if stop_stage == Stage::Explicated {
            let fmt = explicate::Formatter::new(&explicate, &explicated, show_casts);
            return write_out(fmt, out_path);
        }

        let heapified = heapify::Builder::build(explicated);
        if stop_stage == Stage::Heapified {
            let fmt = explicate::Formatter::new(&explicate, &heapified, show_casts);
            return write_out(fmt, out_path)
        }

        let trans_unit = raise::Builder::build(heapified);
        if stop_stage == Stage::Raised {
            let fmt = explicate::Formatter::new(&explicate, &trans_unit, show_casts);
            return write_out(fmt, out_path)
        }

//        let mut flattener = flatten::Flattener::from(explicate);
//        let flattened = heapified.flatten(&mut flattener);
//        if stop_stage == Stage::Flattened {
//            let fmt = flatten::Formatter::new(&flattener, &flattened);
//            return write_out(fmt, out_path)
//        }

        Ok(())
    }
}

impl Stage {
    pub fn file_ext(&self) -> &str {
        use self::Stage::*;
        match *self {
            Ast => "ast",
            Explicated => "explicated",
            Heapified => "heapified",
            Raised => "raised",
            Flattened => "flattened",
            VAsm => "vasm",
            Liveness => "liveness",
            Asm => "s",
            Obj => "o",
            Bin => "bin",
        }
    }
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
        .truncate(true)
        .open(path)
        .chain_err(|| format!("creating file {:?}", path.display()))?;
    write!(f, "{}", data).chain_err(|| "writing data")?;
    Ok(())
}

pub fn link<P1, P2, P3>(asm: P1, runtime: P2, output: P3) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    use std::process::Command;

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
            Err(ErrorKind::LinkRuntime(e).into())
        } else {
            Ok(())
        })
}

fn write_out<D: fmt::Display>(data: D, out_path: &Path) -> Result<()> {
    write_file(data, out_path, false).chain_err(|| {
        format!("Could not write output to {:?}", out_path.display())
    })
}
