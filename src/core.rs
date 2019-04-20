use std::{fmt, io};
use super::escape::{write_escaped_attr, write_escaped_pcdata};


//------------ Content --------------------------------------------------------

/// HTML Content.
///
/// This trait represents anything that can be transformed into HTML content,
/// i.e., a sequence of nested elements and plain character data.
pub trait Content: Sized {
    /// Writes the content.
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error>;

    fn into_string(self) -> String {
        let mut res = Vec::new();
        self.write(&mut res).unwrap();
        String::from_utf8(res).unwrap()
    }
}

impl Content for () {
    fn write<W: io::Write>(self, _target: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}

impl<C1, C2> Content for (C1, C2)
where C1: Content, C2: Content {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        self.0.write(target)?;
        self.1.write(target)
    }
}


impl<T: Content> Content for Option<T> {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        match self {
            Some(writer) => writer.write(target),
            None => Ok(())
        }
    }
}

impl<'a> Content for &'a str {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write_escaped_pcdata(self, target)
    }
}

impl Content for String {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        Content::write(self.as_str(), target)
    }
}

impl Content for url::Url {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        Content::write(self.as_str(), target)
    }
}


//------------ AttributeValue ------------------------------------------------

/// The value of an attribute.
///
/// This trait represents anything that can be transformed into the content
/// of an attribute of an HTML element.
pub trait AttributeValue: Sized {
    /// Writes the value.
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error>;
}

impl AttributeValue for () {
    fn write<W: io::Write>(self, _target: &mut W) -> Result<(), io::Error> {
        Ok(())
    }
}

impl<V1, V2> AttributeValue for (V1, V2)
where V1: AttributeValue, V2: AttributeValue {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        self.0.write(target)?;
        self.1.write(target)
    }
}

impl<T: AttributeValue> AttributeValue for Option<T> {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        match self {
            Some(writer) => writer.write(target),
            None => Ok(())
        }
    }
}

impl<'a> AttributeValue for &'a str {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write_escaped_attr(self, target)
    }
}

impl AttributeValue for String {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        AttributeValue::write(self.as_str(), target)
    }
}

impl AttributeValue for url::Url {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        AttributeValue::write(self.as_str(), target)
    }
}


//------------ Text ----------------------------------------------------------

/// Something that can be both attribute data or content.
///
/// This is useful for functions that return something that can be used for
/// both.
pub trait Text: AttributeValue + Content { }

impl Text for () { }
impl<T1: Text, T2: Text> Text for (T1, T2) { }
impl<T: Text> Text for Option<T> { }
impl<'a> Text for &'a str { }
impl Text for String { }
impl Text for url::Url { }

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
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write!(target, "<{}", self.tag.as_ref())?;
        self.attrs.write(target)?;
        write!(target, ">")?;
        self.content.write(target)?;
        write!(target, "</{}>", self.tag.as_ref())
    }
}


//------------ EmptyElement --------------------------------------------------

pub struct EmptyElement<T, A> {
    tag: T,
    attrs: A,
}

impl<T, A> EmptyElement<T, A> {
    /// Creates a new element from a tag, attributes, and content.
    pub fn new(
        tag: T,
        attrs: A,
    ) -> Self {
        EmptyElement { tag, attrs }
    }
}

impl<T: AsRef<str>, A: Attributes> Content for EmptyElement<T, A> {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write!(target, "<{}", self.tag.as_ref())?;
        self.attrs.write(target)?;
        write!(target, ">")
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

impl<K: AsRef<str>, V: AttributeValue> Attributes for Attr<K, V> {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write!(target, " {}=\"", self.key.as_ref())?;
        self.value.write(target)?;
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
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        target.write_all(self.content.as_ref().as_bytes())
    }
}


//------------ Display -------------------------------------------------------

pub fn display<T>(t: T) -> Display<T> {
    Display(t)
}

#[derive(Clone, Debug)]
pub struct Display<T>(T);

impl<T: fmt::Display> Content for Display<T> {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        write!(target, "{}", self.0)
    }
}


//------------ Iter ----------------------------------------------------------

pub fn iter<I>(iter: I) -> Iter<I> {
    Iter(iter)
}

pub struct Iter<I>(I);

impl<I> Content for Iter<I>
where I: Iterator, I::Item: Content {
    fn write<W: io::Write>(self, target: &mut W) -> Result<(), io::Error> {
        for item in self.0 {
            item.write(target)?;
        }
        Ok(())
    }
}

