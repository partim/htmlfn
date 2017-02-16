//! Formatting traits.

use std::io;
use xml::escape::{escape_str_attribute, escape_str_pcdata};

//------------ AttributeValue ------------------------------------------------

pub trait AttributeValue {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()>;
}

impl AttributeValue for str {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        wr.write_all(escape_str_attribute(self).as_bytes())
    }
}

impl AttributeValue for String {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        wr.write_all(escape_str_attribute(self).as_bytes())
    }
}

impl AttributeValue for bool {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        if *self {
            wr.write_all(b"true")
        }
        else {
            wr.write_all(b"false")
        }
    }
}

impl AttributeValue for i64 {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        write!(wr, "{}", self)
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

