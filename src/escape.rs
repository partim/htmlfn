//! Escaping characters.

use std::fmt;
use std::fmt::Write;
use crate::core::Target;

fn replace_attr_char(ch: char) -> Option<&'static str> {
    match ch {
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&apos;"),
        '&' => Some("&amp;"),
        _ => None
    }
}

fn replace_pcdata_char(ch: char) -> Option<&'static str> {
    match ch {
        '<' => Some("&lt;"),
        '&' => Some("&amp;"),
        _ => None
    }
}

pub fn render_attr(s: &str, target: &mut Target) {
    render_escaped(s, target, replace_attr_char)
}

pub fn render_pcdata(s: &str, target: &mut Target) {
    render_escaped(s, target, replace_pcdata_char)
}

pub fn format_attr(args: fmt::Arguments, target: &mut Target) {
    WriteEscaped { target, op: replace_attr_char }.write_fmt(args).unwrap()
}

pub fn format_pcdata(args: fmt::Arguments, target: &mut Target) {
    WriteEscaped { target, op: replace_pcdata_char }.write_fmt(args).unwrap()
}


fn render_escaped(
    mut s: &str,
    target: &mut Target,
    op: impl Fn(char) -> Option<&'static str>
)  {
    while !s.is_empty() {
        let mut iter = s.char_indices().map(|(idx, ch)| (idx, op(ch)));
        let end = loop {
            match iter.next() {
                Some((idx, Some(repl))) => {
                    // Write up to index, write replacement string,
                    // break with index.
                    target.append_slice(s[0..idx].as_bytes());
                    target.append_slice(repl.as_bytes());
                    break idx;
                }
                Some((_, None)) => { }
                None => {
                    target.append_slice(s.as_bytes());
                    return;
                }
            }
        };
        s = &s[end + 1..];
    }
}


struct WriteEscaped<'a, F> {
    target: &'a mut Target,
    op: F
}

impl<'a, F> fmt::Write for WriteEscaped<'a, F>
where
    F: Fn(char) -> Option<&'static str>
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        render_escaped(s, self.target, &self.op);
        Ok(())
    }
}

