
use std::io;
use super::format::AttributeValue;

macro_rules! attribute {
    ( $name:ident ) => {
        fn $name<V: AttributeValue>(mut self, value: V) -> io::Result<Self> {
            self.writer_mut().write_all(concat!(" ", stringify!($name), "=\"")
                                        .as_bytes())?;
            value.write_all(self.writer_mut())?;
            self.writer_mut().write_all(b"\"")?;
            Ok(self)
        }
    };

    ( $name:ident, $valuetype:ty ) => {
        fn $name(mut self, value: $valuetype) -> io::Result<Self> {
            self.writer_mut().write_all(concat!(" ", stringify!($name), "=\"")
                                        .as_bytes())?;
            value.write_all(self.writer_mut())?;
            self.writer_mut().write_all(b"\"")?;
            Ok(self)
        }
    }
}


//------------ GlobalAttributes ----------------------------------------------

/// Extension trait for element types that support the global attributes.
pub trait GlobalAttributes<W: io::Write>: Sized {
    fn writer_mut(&mut self) -> &mut W;

    attribute!(accesskey);
    attribute!(class);
    attribute!(contenteditable, bool);
    attribute!(contextmenu);

    // XXX key should be something that is guaranteed to be a valid identifier
    fn data<V: AttributeValue>(mut self, key: &str, value: V)
                               -> io::Result<Self> {
        self.writer_mut().write_all(b" data-")?;
        self.writer_mut().write_all(key.as_bytes())?;
        self.writer_mut().write_all(b"=\"")?;
        value.write_all(self.writer_mut())?;
        self.writer_mut().write_all(b"\"")?;
        Ok(self)
    }

    attribute!(dir, Dir);
    #[cfg(experimental_html)] attribute!(draggable, bool);
    #[cfg(experimental_html)] attribute!(dropzone, Dropzone);
    attribute!(hidden, bool);
    attribute!(id);
    #[cfg(experimental_html)] attribute!(itemid);
    #[cfg(experimental_html)] attribute!(itemprop);
    #[cfg(experimental_html)] attribute!(itemref);
    #[cfg(experimental_html)] attribute!(itemscrope);
    #[cfg(experimental_html)] attribute!(itemtype);
    attribute!(lang);
    #[cfg(experimental_html)] attribute!(slot);
    attribute!(spellcheck, bool);
    attribute!(style);
    attribute!(tabindex, i64);
    attribute!(title);
}


//------------ Dir -----------------------------------------------------------

pub enum Dir {
    Ltr,
    Rtl,
    Auto
}

impl AttributeValue for Dir {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        wr.write_all(match *self {
            Dir::Ltr => b"ltr",
            Dir::Rtl => b"rtl",
            Dir::Auto => b"auto",
        })
    }
}


//------------ Dropzone ------------------------------------------------------


#[cfg(experimental_html)]
pub enum Dropzone {
    Copy,
    Move,
    Link,
}

#[cfg(experimental_html)]
impl AttributeValue for Dropzone {
    fn write_all<W: io::Write>(&self, wr: &mut W) -> io::Result<()> {
        wr.write_all(match *self {
            Dropzone::Copy => b"copy",
            Dropzone::Move => b"move",
            Dropzone::Link => b"link",
        })
    }
}

