use vasm::Reg;
use vasm::Lval;
use vasm::Inst;
use vasm::Function;
use vasm::Block;
use vasm;
use explicate::Var;
use std::collections::HashSet;
use std::fmt;

// Removes ebp and esp from liveset, because we don't
// use them for register allocation, ever
fn remove_special_regs(mut live_set: HashSet<Lval>) -> HashSet<Lval> {
    const SPECIAL_REGS: &'static [Reg] = &[Reg::EBP, Reg::ESP];
    for &reg in SPECIAL_REGS {
        let reg = reg.into();
        live_set.remove(&reg);
    }
    live_set
}

pub fn liveness(function: &Function) -> (HashSet<Lval>, Vec<LiveSet>) {
    block_liveness(&function.block, HashSet::new())
}

/// Computes live sets for each instruction
/// in a function
/// Takes live_sets param to retain context of
/// the block following this one (if there is one)
/// Returns (live_set_before_block, live_sets_for_block_insts)
pub fn block_liveness(block: &Block, mut live_after: HashSet<Lval>) -> (HashSet<Lval>, Vec<LiveSet>) {
    // I think this should be initialized to live after? For the case of an empty block?
    let mut live_before: HashSet<Lval> = live_after.clone();
    let mut live_sets: Vec<LiveSet> = Vec::new();

    for inst in block.insts.iter().rev() {
        use self::Inst::*;

        if let If(cond, ref then, ref else_) = *inst {
            let cond = hash_set!(cond);
            let (then_before, then_live_sets) = block_liveness(then, live_after.clone());
            let (else_before, else_live_sets) = block_liveness(else_, live_after.clone());
            live_before = &(&then_before | &else_before) | &cond;
            live_sets.push(LiveSet::If {
                inst: inst,
                then_before: then_before,
                else_before: else_before,
                live_after: live_after.clone(),
                then: then_live_sets,
                else_: else_live_sets,
            });

            live_after = live_before.clone();
            continue;
        }
        if let While(cond, ref header, ref body) = *inst {
            let (body_before, body_live_sets) = block_liveness(body, live_after.clone());
            // live after set of the loop header is the body_before live set
            let (header_before, header_live_sets) = block_liveness(header, body_before.clone());
            let live_before_while = header_before.clone();
            live_sets.push(LiveSet::While {
                inst: inst,
                header_before: header_before,
                body_before: body_before,
                live_after: live_after,
                header: header_live_sets,
                body: body_live_sets,
            });

            live_after = live_before_while;
            continue;
        }
        let w = remove_special_regs(inst.write_set());
        let r = remove_special_regs(inst.read_set());

        live_before = (&live_after - &w)
            .union(&r)
            .map(|&lval| lval)
            .collect();

        live_sets.push(LiveSet::Inst {
            inst: inst,
            live_after: live_after.clone(),
        });

        live_after = live_before.clone();
    }

    live_sets.reverse();
    (live_before, live_sets)
}

#[derive(Debug, Clone)]
pub enum LiveSet<'inst> {
    Inst {
        // Not Inst::If
        inst: &'inst Inst,
        live_after: HashSet<Lval>,
    },
    If {
        // Inst::If
        inst: &'inst Inst,
        then_before: HashSet<Lval>,
        else_before: HashSet<Lval>,
        live_after: HashSet<Lval>,
        then: Vec<LiveSet<'inst>>,
        else_: Vec<LiveSet<'inst>>,
    },
    While {
        // Inst::While
        inst: &'inst Inst,
        header_before: HashSet<Lval>,
        body_before: HashSet<Lval>,
        live_after: HashSet<Lval>,
        header: Vec<LiveSet<'inst>>,
        body: Vec<LiveSet<'inst>>,
    },
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

        match *self {
            LiveSet::Inst {
                inst,
                ref live_after,
            } => {
                f.fmt(inst)?;
                write_live_set(f, live_after)?;
            }
            LiveSet::If {
                inst,
                ref live_after,
                ref then_before,
                ref else_before,
                ref then,
                ref else_,
            } => {
                let cond = if let Inst::If(cond, _, _) = *inst {
                    cond
                } else {
                    panic!()
                };
                writeln!(f, "if {} {{", cond)?;
                f.indent();
                write_live_set(f, then_before)?;
                for live_set in then {
                    f.fmt(live_set)?;
                }
                f.dedent();
                writeln!(f, "}} else {{")?;
                f.indent();
                write_live_set(f, else_before)?;
                for live_set in else_ {
                    f.fmt(live_set)?;
                }
                f.dedent();
                writeln!(f, "}}")?;
                write_live_set(f, live_after)?;
            }
            LiveSet::While {
                inst,
                ref header_before,
                ref body_before,
                ref live_after,
                ref header,
                ref body,
            } => {
                let cond = if let Inst::While(cond, _, _) = *inst {
                    cond
                } else {
                    panic!()
                };
                writeln!(f, "loop {{")?;
                f.indent();
                write_live_set(f, header_before)?;
                for live_set in header {
                    f.fmt(live_set)?;
                }
                writeln!(f, "if {} {{", cond)?;
                f.indent();
                write_live_set(f, body_before)?;
                for live_set in body {
                    f.fmt(live_set)?;
                }
                f.dedent();
                writeln!(f, "}} else {{");
                f.indent();
                writeln!(f, "break;")?;
                f.dedent();
                f.dedent();
                writeln!(f, "}}")?;
                write_live_set(f, live_after)?;
            }
        }
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

        let (before, live_sets) = liveness(self.data);
        write_live_set(f, &before)?;
        for live_set in &live_sets {
            f.fmt(live_set)?;
        }

        Ok(())
    }
}

fn write_live_set<F>(f: &mut F, set: &HashSet<Lval>) -> ::std::io::Result<()>
where
    F: ::std::io::Write,
{
    use std::io::Write;

    write!(f, "live: (")?;
    let live_after: Vec<Lval> = set.iter().map(|&lval| lval).collect();
    if !live_after.is_empty() {
        write!(f, "{}", live_after[0])?;
        for lval in &live_after[1..] {
            write!(f, ", {}", lval)?;
        }
    }
    writeln!(f, ")\n")?;
    Ok(())
}
