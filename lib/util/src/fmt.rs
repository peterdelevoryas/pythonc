use std::fmt;
use std::io;

pub trait Fmt {
    fn fmt<W>(&self, f: &mut Formatter<W>) -> io::Result<()>
    where W: io::Write;
}

pub struct Formatter<W>
where
    W: io::Write
{
    target: W,
    indent: u32,
    // just saw end of line b'\n'
    eol: bool,
}

impl<W> Formatter<W>
where
    W: io::Write
{
    pub fn new(target: W) -> Self {
        Formatter {
            target,
            indent: 0,
            eol: true,
        }
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) {
        self.indent -= 1;
    }

    pub fn fmt<T>(&mut self, arg: &T) -> io::Result<()>
    where
        T: Fmt,
    {
        arg.fmt(self)
    }

    fn write_indent(&mut self) -> io::Result<()> {
        for _ in 0..self.indent {
            self.target.write(b"    ")?;
        }
        Ok(())
    }

    fn write_byte(&mut self, b: u8) -> io::Result<()> {
        self.target.write(&[b])?;
        Ok(())
    }
}

impl<W> io::Write for Formatter<W>
where
    W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &b in buf {
            match b {
                b'\n' if self.eol => {
                    self.write_byte(b)?;
                }
                b'\n' if !self.eol => {
                    self.write_byte(b)?;
                    self.eol = true;
                }
                b if self.eol => {
                    self.write_indent()?;
                    self.write_byte(b)?;
                    self.eol = false;
                }
                b if !self.eol => {
                    self.write_byte(b)?;
                }
                _ => unreachable!()
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.target.flush()
    }
}
