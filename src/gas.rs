use std::io;
use vm::Visit;
use vm;

struct Writer<'dst, W>
where
    W: io::Write + 'dst,
{
    dst: &'dst mut W,
}

impl<'dst, W> Visit for Writer<'dst, W>
where
    W: io::Write + 'dst
{
}

pub fn write_gas<W>(dst: &mut W, vm: &vm::Module)
where
    W: io::Write,
{
    let mut writer = Writer { dst };
    writer.visit_module(vm);
}
