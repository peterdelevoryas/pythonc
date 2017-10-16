extern crate python_token as token;
extern crate python_ast as ast;
extern crate python_ir as ir;
extern crate python_trans as trans;
extern crate python_vm as vm;
extern crate parser;
extern crate interference;
extern crate liveness;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod error;
pub use error::Error;
pub use error::ErrorKind;
pub use error::Result;
pub use error::ResultExt;

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::fmt;

#[derive(Debug)]
pub struct Compiler {
    runtime: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum CompilerStage {
    PyRepr,
    Ast,
    Ir,
    Vm,
    Asm,
    Obj,
    Bin,
}

#[derive(Debug)]
pub struct PyRepr(String);

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { runtime: None }
    }

    pub fn with_runtime(runtime: PathBuf) -> Compiler {
        Compiler { runtime: Some(runtime) }
    }

    pub fn emit(
        &self,
        in_path: &Path,
        stage: CompilerStage,
        out_path: Option<&Path>,
    ) -> Result<()> {
        let source = read_file(in_path).chain_err(|| {
            format!("Could not read input file {:?}", in_path.display())
        })?;
        let py_repr = self.emit_py_repr(&source).chain_err(|| {
            format!("Could not create Python repr of source")
        })?;
        if stage == CompilerStage::PyRepr {
            let py_repr_path = default_out_path(&in_path, CompilerStage::PyRepr);
            let py_repr = format!("{:#?}", py_repr);
            return write_out(py_repr, &py_repr_path)
        }
        let ast = self.emit_ast(py_repr).chain_err(
            || "Could not create AST from source",
        )?;
        if stage == CompilerStage::Ast {
            // Ast doesn't implement Display, but it does implement debug,
            // so first write in repr format to string
            let ast = format!("{:#?}", ast);
            let ast_path = default_out_path(&in_path, CompilerStage::Ast);
            return write_out(ast, &ast_path);
        }
        let mut tmp_allocator = ir::TmpAllocator::new();
        let ir = self.emit_ir(ast, &mut tmp_allocator).chain_err(
            || "Could not create IR from AST",
        )?;
        if stage == CompilerStage::Ir {
            let ir_path = default_out_path(&in_path, CompilerStage::Ir);
            return write_out(ir, &ir_path);
        }
        let vm = self.emit_vm(ir).chain_err(
            || "Could not create virtual assembly from IR",
        )?;
        if stage == CompilerStage::Vm {
            let vm_path = default_out_path(&in_path, CompilerStage::Vm);
            return write_out(vm, &vm_path);
        }
        let asm = self.emit_asm(vm, &mut tmp_allocator).chain_err(
            || "Could not create assembly from virtual assembly",
        )?;
        if stage == CompilerStage::Asm {
            let asm_path = default_out_path(&in_path, CompilerStage::Asm);
            return write_out(asm, &asm_path);
        }

        let obj_path = default_out_path(&in_path, CompilerStage::Obj);
        self.emit_obj(asm, &obj_path).chain_err(
            || "Could not create object file from virtual assembly",
        )?;

        let runtime = match self.runtime {
            Some(ref lib) => lib,
            None => return Err(ErrorKind::MissingRuntime.into()),
        };

        let bin_path = default_out_path(&in_path, CompilerStage::Bin);
        self.emit_bin(&obj_path, &runtime, &bin_path)
    }

    pub fn emit_py_repr(&self, source: &str) -> Result<PyRepr> {
        python_repr(source).chain_err(|| "Could not get Python repr of source")
    }

    pub fn emit_ast(&self, py_repr: PyRepr) -> Result<ast::Program> {
        parser::parse_program(py_repr.0.as_bytes())
            .map_err(Error::from)
            .chain_err(|| "Error parsing program from official Python parser")
    }

    pub fn emit_ir(
        &self,
        ast: ast::Program,
        tmp_allocator: &mut ir::TmpAllocator,
    ) -> Result<ir::Program> {
        Ok(ir::Builder::build(ast, tmp_allocator))
    }

    pub fn emit_vm(&self, ir: ir::Program) -> Result<vm::Program> {
        Ok(vm::Program::build(&ir))
    }

    // TODO Needs more typing (asm::Program should be returned)
    pub fn emit_asm(
        &self,
        vm: vm::Program,
        tmp_allocator: &mut ir::TmpAllocator,
    ) -> Result<vm::Program> {
        let mut vm = vm;
        let asm;
        let mut iteration = 0;
        loop {
            use interference::DSaturResult::*;
            //println!("iteration {}", iteration);
            //liveness::debug_print(&vm);

            let mut ig = interference::Graph::build(&vm);
            match ig.run_dsatur() {
                Success => {
                    asm = ig.assign_homes(vm);
                    //println!("asm:\n{}", asm);
                    break;
                }
                Spill(u) => {
                    // replaces u with stack_index
                    vm.spill(u);
                    vm = vm.replace_stack_to_stack_ops(tmp_allocator);
                }
            }
            iteration += 1;
        }
        Ok(asm)
    }

    // TODO Needs better typing (asm should have its own type)
    pub fn emit_obj(&self, asm: vm::Program, out: &Path) -> Result<()> {
        use std::process::Stdio;
        use std::io::Write;

        let asm = format!("{}", asm);
        let mut gcc = Command::new("gcc")
            .args(&["-m32", "-g", "-c"])
            .arg("-o")
            .arg(out.as_os_str())
            .args(&["-xassembler", "-"])
            .stdin(Stdio::piped())
            .spawn()
            .chain_err(|| "Could not spawn gcc assembler")?;
        match gcc.stdin {
            Some(ref mut stdin) => {
                stdin.write_all(asm.as_bytes()).chain_err(
                    || "Could not write assembly to gcc through pipe",
                )?;
            }
            None => return Err(ErrorKind::Msg("Could not capture gcc stdin".into()).into()),
        }
        gcc.wait()
            .chain_err(|| "Error running gcc assembler")
            .and_then(|exit_code| if !exit_code.success() {
                Err(ErrorKind::Assembler(exit_code).into())
            } else {
                Ok(())
            })
    }

    pub fn emit_bin(&self, obj: &Path, runtime: &Path, out: &Path) -> Result<()> {
        Command::new("gcc")
            .args(&["-m32", "-g"])
            .arg(obj.as_os_str())
            .arg(runtime.as_os_str())
            .arg("-o")
            .arg(out.as_os_str())
            .spawn()
            .chain_err(|| "Could not spawn gcc linker")?
            .wait()
            .chain_err(|| "Error running gcc linker")
            .and_then(|exit_code| if !exit_code.success() {
                Err(ErrorKind::LinkRuntime(exit_code).into())
            } else {
                Ok(())
            })
    }

    /*
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
    */

    /*
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
    */
}

fn write_out<D: fmt::Display>(data: D, out_path: &Path) -> Result<()> {
    write_file(data, out_path, false).chain_err(|| {
        format!("Could not write output to {:?}", out_path.display())
    })
}

fn default_out_path(input: &Path, stage: CompilerStage) -> PathBuf {
    let extension = match stage {
        CompilerStage::PyRepr => "py_repr",
        CompilerStage::Ast => "ast",
        CompilerStage::Ir => "ir",
        CompilerStage::Vm => "vm",
        CompilerStage::Asm => "s",
        CompilerStage::Obj => "o",
        CompilerStage::Bin => "bin",
    };
    input.with_extension(extension)
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

/*
pub fn compile(source: &[u8]) -> Result<vm::Program> {
    /*
    #[cfg(not(feature = "fallback-parser"))]
    {
        let tokens = token::Stream::new(source);
        let ast = ast::parse_program(tokens).chain_err(|| "parse error")?;
    }
    */

    let ast = parser::parse_program(source).
        map_err(Error::from)
        .chain_err(|| "parse error")?;
    println!("parsed ast: {:#?}", ast);

    /*
    let ast = python_fallback_parser::parse_program_fallback(source)
        .map_err(|e| format!("{:?}", e))
        .map_err(Error::from)
        .chain_err(|| "parse error")?;
    */

    let mut tmp_allocator = ir::TmpAllocator::new();
    let ir: ir::Program = ir::Builder::build(ast, &mut tmp_allocator);
    //ir::debug_print(ir.stmts.iter());

    let mut vm = vm::Program::build(&ir);
    let asm;
    let mut iteration = 0;
    loop {
        use interference::DSaturResult::*;
        println!("iteration {}", iteration);
        liveness::debug_print(&vm);

        let mut ig = interference::Graph::build(&vm);
        match ig.run_dsatur() {
            Success => {
                asm = ig.assign_homes(vm);
                println!("asm:\n{}", asm);
                break
            }
            Spill(u) => {
                // replaces u with stack_index
                vm.spill(u);
                vm = vm.replace_stack_to_stack_ops(&mut tmp_allocator);
            }
        }
        iteration += 1;
    }

    //let asm = trans::Program::build(&ir);
    Ok(asm)
}
*/

pub fn python_repr(source: &str) -> Result<PyRepr> {
    use std::process::Command;
    use std::process::Stdio;

    let mut parser = Command::new("python")
        // TODO should be configurable where this is
        .arg("parse.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .chain_err(|| "Could not spawn parse.py")?;
    match parser.stdin {
        Some(ref mut stdin) => {
            use std::io::Write;
            stdin.write_all(source.as_bytes()).chain_err(
                || "Could not write source bytes to parse.py",
            )?;
        }
        None => {
            return Err(
                ErrorKind::Msg(
                    "parse.py stdin is not being captured, cannot send source to it".into(),
                ).into(),
            )
        }
    }
    let output = parser.wait_with_output().chain_err(
        || "Could not capture parse.py output",
    )?;
    let repr = String::from_utf8(output.stdout).chain_err(
        || "parse.py output is not valid utf-8",
    )?;

    Ok(PyRepr(repr))
}

pub fn parse_source<P>(source: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    use std::process::Command;
    let source = source.as_ref();
    let output = Command::new("python")
        .arg("parse.py")
        .arg(source)
        .output()
        .chain_err(|| {
            format!("Error running python parse.py {}", source.display())
        })?;
    Ok(output.stdout)
}

/*
pub fn emit_asm<P1, P2>(source: P1, output: P2, create_new: bool) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let source = source.as_ref();
    //let source = read_file(source).chain_err(|| "reading source file")?;
    let python_repr = parse_source(source).chain_err(|| "parsing source")?;
    let asm = compile(&python_repr).chain_err(|| {
        format!("compiling source file {:?}", source.display())
    })?;

    write_file(asm, output, create_new)
}
*/

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
            Err(ErrorKind::LinkRuntime(e).into())
        } else {
            Ok(())
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
