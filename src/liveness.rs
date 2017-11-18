use vasm::Reg;
use vasm::Lval;
use vasm::Inst;
use vasm::Function;
use vasm::Block;
use vasm;
use explicate::Var;
use std::collections::HashSet;
use std::fmt;

/// Computes live sets for each instruction
/// in a function
pub fn liveness(block: &Block) -> Vec<LiveSet> {
    let mut live_sets: Vec<LiveSet> = Vec::new();
    let mut live_after: HashSet<Lval> = HashSet::new();
    let mut live_before: HashSet<Lval>;

    for inst in block.insts.iter().rev() {
        use self::Inst::*;

        if let If(cond, ref then, ref else_) = *inst {
            unimplemented!("if statement liveness unimplemented")
        }
        let w = inst.write_set();
        let r = inst.read_set();

        live_before = (&live_after - &w)
            .union(&r)
            .map(|&lval| lval)
            .collect();

        live_sets.push(LiveSet {
            inst: inst,
            live_after: live_after.clone(),
        });

        live_after = live_before;
    }

    live_sets.reverse();
    live_sets
}

#[derive(Debug, Clone)]
pub struct LiveSet<'inst> {
    pub inst: &'inst Inst,
    pub live_after: HashSet<Lval>,
}

pub fn liveset_debug_string(vasm: &vasm::Module) -> String {
    let formatter = Formatter { data: vasm };
    let mut f = ::util::fmt::Formatter::new(Vec::new());
    f.fmt(&formatter).unwrap();
    String::from_utf8(f.into_inner()).unwrap()
}

struct Formatter<'a, T: 'a> {
    data: &'a T,
}

impl<'a> ::util::fmt::Fmt for Formatter<'a, vasm::Module> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::std::io::Result<()>
    where
        W: ::std::io::Write
    {
        use std::io::Write;
        for (&func, function) in &self.data.funcs {
            if func == self.data.main {
                writeln!(f, ".globl main")?;
                writeln!(f, "main:")?;
            } else {
                writeln!(f, "{}:", func)?;
            }
            f.indent();
            f.fmt(&Formatter { data: function });
            f.dedent();
        }
        Ok(())
    }
}

impl<'inst> ::util::fmt::Fmt for LiveSet<'inst> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::std::io::Result<()>
    where
        W: ::std::io::Write
    {
        use std::io::Write;
        writeln!(f, "{}", self.inst)?;
        write!(f, "live: (")?;
        let live_after: Vec<Lval> = self.live_after.iter().map(|&lval| lval).collect();
        if !live_after.is_empty() {
            write!(f, "{}", live_after[0])?;
            for lval in &live_after[1..] {
                write!(f, ", {}", lval)?;
            }
        }
        writeln!(f, ")\n")?;
        Ok(())
    }
}

impl<'a> ::util::fmt::Fmt for Formatter<'a, vasm::Function> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::std::io::Result<()>
    where
        W: ::std::io::Write
    {
        use std::io::Write;
        writeln!(f, "push %ebp")?;
        writeln!(f, "mov %esp, %ebp")?;
        writeln!(f, "sub ${}, %esp", self.data.stack_slots as vasm::Imm * vasm::WORD_SIZE)?;
        f.fmt(&Formatter { data: &self.data.block });
        Ok(())
    }
}

impl<'a> ::util::fmt::Fmt for Formatter<'a, vasm::Block> {
    fn fmt<W>(&self, f: &mut ::util::fmt::Formatter<W>) -> ::std::io::Result<()>
    where
        W: ::std::io::Write
    {
        use std::io::Write;
        let live_sets = liveness(self.data);

        for live_set in &live_sets {
            match *live_set.inst {
                vasm::Inst::If(cond, ref then, ref else_) => {
                    writeln!(f, "if {} {{", cond)?;
                    f.indent();
                    f.fmt(&Formatter { data: then })?;
                    f.dedent();
                    writeln!(f, "}} else {{")?;
                    f.indent();
                    f.fmt(&Formatter { data: else_ })?;
                    f.dedent();
                    writeln!(f, "}}")?;
                }
                _ => {
                    f.fmt(live_set)?;
                }
            }
        }
        Ok(())
    }
}
