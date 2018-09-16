//! Escaping characters.

use std::io;

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

fn write_escaped<W: io::Write, F>(
    mut s: &str,
    target: &mut W,
    op: F
) -> Result<(), io::Error>
where F: Fn(char) -> Option<&'static str> {
    while !s.is_empty() {
        let mut iter = s.char_indices().map(|(idx, ch)| (idx, op(ch)));
        let end = loop {
            match iter.next() {
                Some((idx, Some(repl))) => {
                    // Write up to index, write replacement string,
                    // break with index.
                    target.write_all(s[0..idx].as_bytes())?;
                    target.write_all(repl.as_bytes())?;
                    break idx;
                }
                Some((_, None)) => { }
                None => {
                    return target.write_all(s.as_bytes());
                }
            }
        };
        s = &s[end + 1..];
    }
    Ok(())
}

pub fn write_escaped_attr<W: io::Write>(
    s: &str,
    target: &mut W
) -> Result<(), io::Error> {
    write_escaped(s, target, replace_attr_char)
}

pub fn write_escaped_pcdata<W: io::Write>(
    s: &str,
    target: &mut W
) -> Result<(), io::Error> {
    write_escaped(s, target, replace_pcdata_char)
}

