//! Formatting traits.

use std::io;
use xml::escape::{escape_str_attribute, escape_str_pcdata};

//------------ AttributeValue ------------------------------------------------

pub trait AttributeValue {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()>;
}

impl<T: AsRef<str>> AttributeValue for T {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        wr.write_all(escape_str_attribute(self.as_ref()).as_bytes())
    }
}


//------------ PCData --------------------------------------------------------

pub trait PCData {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()>;
}

impl<T: AsRef<str>> PCData for T {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        wr.write_all(escape_str_pcdata(self.as_ref()).as_bytes())
    }
}

