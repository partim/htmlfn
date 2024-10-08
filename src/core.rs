use std::fmt;
use super::escape;


//============ Traits ========================================================

//------------ Content -------------------------------------------------------

/// HTML Content.
///
/// This trait represents anything that can be transformed into HTML content,
/// i.e., a sequence of nested elements and plain character data.
pub trait Content {
    fn render_content(self, target: &mut Target);

    fn render(self) -> Target
    where Self: Sized {
        let mut target = Target::new();
        self.render_content(&mut target);
        target
    }
}

impl<'a> Content for &'a str {
    fn render_content(self, target: &mut Target) {
        escape::render_pcdata(self, target)
    }
}

impl Content for String {
    fn render_content(self, target: &mut Target) {
        self.as_str().render_content(target)
    }
}

impl<'a> Content for fmt::Arguments<'a> {
    fn render_content(self, target: &mut Target) {
        target.write_fmt(self)
    }
}
    

impl Content for url::Url {
    fn render_content(self, target: &mut Target) {
        self.as_str().render_content(target)
    }
}

impl Content for Target {
    fn render_content(self, target: &mut Target) {
        target.append_slice(self.as_ref())
    }
}


//------------ AttributeName -------------------------------------------------

/// The name of an attribute.
///
/// This trait represents anything that can be transformed into the name
/// of an attribute of an HTML element. The trait is necessary because we
/// might want to construct the names of data attributes from prefix and
/// suffix.
pub trait AttributeName {
    fn render_attr_name(self, target: &mut Target);
}

impl<'a> AttributeName for &'a str {
    fn render_attr_name(self, target: &mut Target) {
        target.append_slice(self.as_bytes())
    }
}


//------------ AttributeValue ------------------------------------------------

/// The value of an attribute.
///
/// This trait represents anything that can be transformed into the content
/// of an attribute of an HTML element.
pub trait AttributeValue {
    fn render_attr_value(self, target: &mut Target);
}

impl<'a> AttributeValue for &'a str {
    fn render_attr_value(self, target: &mut Target) {
        escape::render_attr(self, target)
    }
}

impl<'a> AttributeValue for fmt::Arguments<'a> {
    fn render_attr_value(self, target: &mut Target) {
        escape::format_attr(self, target)
    }
}

//------------ Attributes ----------------------------------------------------

/// Attributes of an HTML element.
///
/// This trait represents a, possibly empty, sequence of HTML element
/// attributes.
pub trait Attributes {
    /// Renders the attributes.
    ///
    /// If the atttribute sequence is not empty, at least one white space
    /// character has to be written before the actual attributes.
    fn render_attrs(self, target: &mut Target);
}


//------------ Tokens --------------------------------------------------------

/// Tokens are short strings that identify things.
pub trait Tokens<'a> {
    type Iter: Iterator<Item = &'a str>;

    fn iter_tokens(self) -> Self::Iter;
}

impl<'a> Tokens<'a> for &'a str {
    type Iter = std::iter::Once<&'a str>;

    fn iter_tokens(self) -> Self::Iter {
        std::iter::once(self)
    }
}

impl<'a, const N: usize> Tokens<'a> for [&'a str; N] {
    type Iter = <[&'a str; N] as IntoIterator>::IntoIter;

    fn iter_tokens(self) -> Self::Iter {
        IntoIterator::into_iter(self)
    }
}

impl<'a> Tokens<'a> for Option<&'a str> {
    type Iter = std::option::IntoIter<&'a str>;

    fn iter_tokens(self) -> Self::Iter {
        self.into_iter()
    }
}


//------------ Text ----------------------------------------------------------

/// Text.
pub trait Text: Content {
}

impl<'a> Text for &'a str {
}

impl Text for () { }

impl<T: Text> Text for Option<T> { }


//============ Types =========================================================

//------------ Target --------------------------------------------------------

/// A buffer to render the HTML data into.
#[derive(Default)]
pub struct Target {
    buf: Vec<u8>,
}

impl Target {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn append_slice(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) {
        fmt::write(self, args).unwrap();
    }
}

impl fmt::Write for Target {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.append_slice(s.as_bytes());
        Ok(())
    }
}

impl AsRef<[u8]> for Target {
    fn as_ref(&self) -> &[u8] {
        self.buf.as_ref()
    }
}

#[cfg(feature = "hyper")]
impl Into<hyper::body::Body> for Target {
    fn into(self) -> hyper::body::Body {
        self.buf.into()
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
    pub fn new(tag: T, attrs: A, content: C) -> Self {
        Element { tag, attrs, content }
    }
}

impl<T: AsRef<str>, A: Attributes, C: Content> Content for Element<T, A, C> {
    fn render_content(self, target: &mut Target) {
        write!(target, "<{}", self.tag.as_ref());
        self.attrs.render_attrs(target);
        write!(target, ">");
        self.content.render_content(target);
        write!(target, "</{}>", self.tag.as_ref())
    }
}


//------------ EmptyElement --------------------------------------------------

/// An HTML element.
pub struct EmptyElement<T, A> {
    tag: T,
    attrs: A,
}

impl<T: AsRef<str>, A: Attributes> EmptyElement<T, A> {
    pub fn new(tag: T, attrs: A) -> Self {
        EmptyElement { tag, attrs }
    }
}

impl<T: AsRef<str>, A: Attributes> Content for EmptyElement<T, A> {
    fn render_content(self, target: &mut Target) {
        write!(target, "<{}", self.tag.as_ref());
        self.attrs.render_attrs(target);
        write!(target, "/>");
    }
}


//------------ TextElement ---------------------------------------------------

/// An HTML element.
pub struct TextElement<T, A, C> {
    tag: T,
    attrs: A,
    content: C
}

impl<T, A, C> TextElement<T, A, C> {
    pub fn new(tag: T, attrs: A, content: C) -> Self {
        TextElement { tag, attrs, content }
    }
}

impl<T: AsRef<str>, A: Attributes, C: Text> Content for TextElement<T, A, C> {
    fn render_content(self, target: &mut Target) {
        write!(target, "<{}", self.tag.as_ref());
        self.attrs.render_attrs(target);
        write!(target, ">");
        self.content.render_content(target);
        write!(target, "</{}>", self.tag.as_ref())
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

impl<K: AttributeName, V: AttributeValue> Attributes for Attr<K, V> {
    fn render_attrs(self, target: &mut Target) {
        target.append_slice(b" ");
        self.key.render_attr_name(target);
        target.append_slice(b"=\"");
        self.value.render_attr_value(target);
        target.append_slice(b"\"");
    }
}


//------------ Raw -----------------------------------------------------------

/// Renders string content without escaping.
///
/// You can use this for instance for the doctype at the beginning of a
/// document.
pub fn raw<C>(content: C) -> Raw<C> {
    Raw { content }
}

pub struct Raw<C> {
    content: C
}

impl<C: AsRef<str>> Content for Raw<C> {
    fn render_content(self, target: &mut Target) {
        target.append_slice(self.content.as_ref().as_bytes())
    }
}


//============ Impl Traits for Tuples ========================================

macro_rules! render_tuple {
    ( $trait:ident, $render:ident) => {
        impl $trait for () {
            fn $render(self, _target: &mut Target) { }
        }

        impl<T: $trait> $trait for Option<T> {
            fn $render(self, target: &mut Target) {
                if let Some(value) = self {
                    value.$render(target)
                }
            }
        }

        impl<T1: $trait> $trait for (T1,) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
            }
        }

        impl<T1: $trait, T2: $trait> $trait for (T1, T2) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
            }
        }

        impl<T1: $trait, T2: $trait, T3: $trait> $trait for (T1, T2, T3) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
        > $trait for (T1, T2, T3, T4) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
        > $trait for (T1, T2, T3, T4, T5) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7, T8) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
                self.7.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
            T9: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
                self.7.$render(target);
                self.8.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
            T9: $trait,
            T10: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
                self.7.$render(target);
                self.8.$render(target);
                self.9.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
            T9: $trait,
            T10: $trait,
            T11: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
                self.7.$render(target);
                self.8.$render(target);
                self.9.$render(target);
                self.10.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
            T9: $trait,
            T10: $trait,
            T11: $trait,
            T12: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
                self.7.$render(target);
                self.8.$render(target);
                self.9.$render(target);
                self.10.$render(target);
                self.11.$render(target);
            }
        }

        impl<
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
            T9: $trait,
            T10: $trait,
            T11: $trait,
            T12: $trait,
            T13: $trait,
        > $trait for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
            fn $render(self, target: &mut Target) {
                self.0.$render(target);
                self.1.$render(target);
                self.2.$render(target);
                self.3.$render(target);
                self.4.$render(target);
                self.5.$render(target);
                self.6.$render(target);
                self.7.$render(target);
                self.8.$render(target);
                self.9.$render(target);
                self.10.$render(target);
                self.11.$render(target);
                self.12.$render(target);
            }
        }
    }
}

render_tuple!(Content, render_content);
render_tuple!(AttributeName, render_attr_name);
render_tuple!(AttributeValue, render_attr_value);
render_tuple!(Attributes, render_attrs);


impl<
    T0: Text
> Text for (T0,) { }

impl<
    T0: Text, T1: Text
> Text for (T0, T1,) { }

impl<
    T0: Text, T1: Text, T2: Text,
> Text for (T0, T1, T2) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text,
> Text for (T0, T1, T2, T3) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
> Text for (T0, T1, T2, T3, T4) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text,
> Text for (T0, T1, T2, T3, T4, T5) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text, T7: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6, T7) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text, T7: Text, T8: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6, T7, T8) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text, T7: Text, T8: Text, T9: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text, T7: Text, T8: Text, T9: Text,
    T10: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text, T7: Text, T8: Text, T9: Text,
    T10: Text, T11: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) { }

impl<
    T0: Text, T1: Text, T2: Text, T3: Text, T4: Text,
    T5: Text, T6: Text, T7: Text, T8: Text, T9: Text,
    T10: Text, T11: Text, T12: Text,
> Text for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) { }

