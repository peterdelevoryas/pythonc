use std::fmt;
use std::io;
use vm::Visit;
use vm::Module;
use vm::FuncData;
use vm::BlockData;
use vm::Inst;
use vm::Term;


pub fn fmt_indented<T>(data: &T) -> String
where
    T: fmt::Display,
{
    let s = format!("{}", data);
    indented(&s)
}

pub fn indented(s: &str) -> String {
    let mut indented = String::new();
    // just saw end of line
    let mut eol = true;
    for c in s.chars() {
        match c {
            '\n' if eol => {
                indented.push(c);
            }
            '\n' if !eol => {
                indented.push(c);
                eol = true;
            }
            c if eol => {
                indented.push_str("    ");
                indented.push(c);
                eol = false;
            }
            c if !eol => {
                indented.push(c);
            }
            _ => unreachable!(),
        }
    }

    return indented;
}

struct Writer<'dst, W>
where
    W: io::Write + 'dst,
{
    dst: &'dst mut W,
}

impl<'dst, W> Visit for Writer<'dst, W>
where
    W: io::Write + 'dst,
{
    fn visit_func(&mut self, func: &FuncData) {
        let r: io::Result<()> = do catch {
            writeln!(
                self.dst,
                "func {name}({args}) {{",
                name=func.name(),
                args=::itertools::join(&func.args, ", ")
            )?;

            self.traverse_func(func);

            writeln!(self.dst, "}}")?;

            Ok(())
        };
        r.unwrap();
    }

    fn visit_block(&mut self, block: &BlockData) {
        let r: io::Result<()> = do catch {
            writeln!(self.dst, "{}:", block.name())?;
            self.traverse_block(block);
            Ok(())
        };
        r.unwrap();
    }

    fn visit_inst(&mut self, inst: &Inst) {
        let r: io::Result<()> = do catch {
            writeln!(self.dst, "    {}", inst)?;
            Ok(())
        };
        r.unwrap();
    }

    fn visit_term(&mut self, term: &Term) {
        let r: io::Result<()> = do catch {
            writeln!(self.dst, "    {}", term)?;
            Ok(())
        };
        r.unwrap();
    }
}

pub fn write<W>(dst: &mut W, module: &Module)
where
    W: io::Write,
{
    let mut writer = Writer { dst };
    writer.visit_module(module);
}
