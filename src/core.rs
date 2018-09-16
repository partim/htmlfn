use std::io;
use super::escape::{write_escaped_attr, write_escaped_pcdata};


//------------ Content --------------------------------------------------------

/// HTML Content.
///
/// This trait represents anything that can be transformed into HTML content,
/// i.e., a sequence of nested elements and plain character data.
pub trait Content {
    /// Returns whether the content is empty.
    ///
    /// This is used to produce a correct empty tag.
    fn is_empty(&self) -> bool;

    /// Writes the content.
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error>;
}

impl Content for () {
    fn is_empty(&self) -> bool {
        true
    }

    fn write<W: io::Write>(self, _target: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}

impl<C1, C2> Content for (C1, C2)
where C1: Content, C2: Content {
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        self.0.write(target)?;
        self.1.write(target)
    }
}


impl<T: Content> Content for Option<T> {
    fn is_empty(&self) -> bool {
        self.is_some()
    }

    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        match self {
            Some(writer) => writer.write(target),
            None => Ok(())
        }
    }
}

impl<'a> Content for &'a str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }

    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write_escaped_pcdata(self, target)
    }
}


//------------ Attributes ----------------------------------------------------

/// Attributes of an HTML element.
///
/// This trait represents a, possibly empty, sequence of HTML element
/// attributes.
pub trait Attributes {
    /// Writes the attributes.
    ///
    /// If the atttribute sequence is not empty, at least one white space
    /// character has to be written before the actual attributes.
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error>;
}

impl Attributes for () {
    fn write<W: io::Write>(self, _target: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}

impl<A1: Attributes, A2: Attributes> Attributes for (A1, A2) {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        self.0.write(target)?;
        self.1.write(target)
    }
}


//------------ Element -------------------------------------------------------

/// An HTML element.
pub struct Element<T, A, C> {
    tag: T,
    attrs: A,
    content: C
}

impl<T, A, C> Element<T, A, C> {
    /// Creates a new element from a tag, attributes, and content.
    pub fn new(
        tag: T,
        attrs: A,
        content: C
    ) -> Element<T, A, C> {
        Element { tag, attrs, content }
    }
}

impl<T: AsRef<str>, A: Attributes, C: Content> Content for Element<T, A, C> {
    fn is_empty(&self) -> bool {
        false
    }

    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write!(target, "<{}", self.tag.as_ref())?;
        self.attrs.write(target)?;
        if self.content.is_empty() {
            write!(target, "/>")?;
        }
        else {
            write!(target, ">")?;
            self.content.write(target)?;
            write!(target, "</{}>", self.tag.as_ref())?;
        }
        Ok(())
    }
}


//------------ Attr ----------------------------------------------------------

/// A single HTML element attribute.
pub struct Attr<K, V> {
    key: K,
    value: V
}

impl<K, V> Attr<K, V> {
    /// Creats a new attribute from a key and a value.
    pub fn new(key: K, value: V) -> Attr<K, V> {
        Attr { key, value }
    }
}

impl<K: AsRef<str>, V: AsRef<str>> Attributes for Attr<K, V> {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write!(target, " {}=\"", self.key.as_ref())?;
        write_escaped_attr(self.value.as_ref(), target)?;
        write!(target, "\"")
    }
}


//------------ Raw -----------------------------------------------------------

/// Raw string content.
///
/// Text wrapped in this struct will not be escaped when written. You can use
/// this for instance for the doctype at the beginning of a document.
pub struct Raw<C> {
    content: C
}

impl<C> Raw<C> {
    /// Creates new raw content.
    pub fn new(content: C) -> Self {
        Raw { content }
    }
}

impl<C: AsRef<str>> Content for Raw<C> {
    fn is_empty(&self) -> bool {
        self.content.as_ref().is_empty()
    }

    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        target.write_all(self.content.as_ref().as_bytes())
    }
}

