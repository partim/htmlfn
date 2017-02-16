
use std::io;
use super::format::AttributeValue;


//------------ GlobalAttributes ----------------------------------------------

/// Extension trait for element types that support the global attributes.
pub trait GlobalAttributes<W: io::Write>: Sized {
    fn writer_mut(&mut self) -> &mut W;

    fn class<V: AttributeValue>(mut self, value: V) -> io::Result<Self> {
        self.writer_mut().write_all(b" class=\"")?;
        value.write_all(self.writer_mut())?;
        self.writer_mut().write_all(b"\"")?;
        Ok(self)
    }
}

