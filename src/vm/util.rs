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
}

pub fn write<W>(dst: &mut W, module: &Module)
where
    W: io::Write,
{
    let mut writer = Writer { dst };
    writer.visit_module(module);
}
