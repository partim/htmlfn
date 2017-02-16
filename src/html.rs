
use std::io;
use super::common::GlobalAttributes;
use super::format::AttributeValue;
use super::metadata::MetadataContent;
use super::flow::FlowContent;


//------------ Html ----------------------------------------------------------

pub struct Html<W: io::Write>(W);

impl<W: io::Write> Html<W> {
    pub fn new(mut wr: W) -> io::Result<Self> {
        wr.write_all(b"<!DOCTYPE html>\n<html")?;
        Ok(Html(wr))
    }

    pub fn xmlns<T: AttributeValue>(mut self, value: T) -> io::Result<Self> {
        self.0.write_all(b" xmlns=\"")?;
        value.write_all(&mut self.0)?;
        self.0.write_all(b"\"")?;
        Ok(self)
    }

    pub fn head(mut self) -> io::Result<Head<W>> {
        self.0.write_all(b">")?;
        Head::new(self.0)
    }
}


//------------ Head ----------------------------------------------------------

pub struct Head<W: io::Write>(W);

impl<W: io::Write> Head<W> {
    fn new(mut wr: W) -> io::Result<Self> {
        wr.write_all(b"<head")?;
        Ok(Head(wr))
    }

    pub fn content<F>(mut self, op: F) -> io::Result<HtmlWithHead<W>>
                   where F: FnOnce(MetadataContent<W>)
                                   -> io::Result<MetadataContent<W>> {
        self.0.write_all(b">")?;
        let mut w = op(MetadataContent::new(self.0))?.into_inner();
        w.write_all(b"</head>")?;
        HtmlWithHead::new(w)
    }
}

impl<W: io::Write> GlobalAttributes<W> for Head<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}


//------------ HtmlWithHead --------------------------------------------------

pub struct HtmlWithHead<W: io::Write>(W);

impl<W: io::Write> HtmlWithHead<W> {
    fn new(wr: W) -> io::Result<Self> {
        Ok(HtmlWithHead(wr))
    }

    pub fn body(self) -> io::Result<Body<W>> {
        Body::new(self.0)
    }
}


//------------ Body ----------------------------------------------------------

pub struct Body<W: io::Write>(W);

impl<W: io::Write> Body<W> {
    fn new(mut wr: W) -> io::Result<Self> {
        wr.write_all(b"<body")?;
        Ok(Body(wr))
    }

    // ...

    pub fn content<F>(mut self, op: F) -> io::Result<W>
                   where F: FnOnce(FlowContent<W>)
                                   -> io::Result<FlowContent<W>> {
        self.0.write_all(b">")?;
        let mut w = op(FlowContent::new(self.0))?.into_inner();
        w.write_all(b"</body></html>")?;
        Ok(w)
    }
}

impl<W: io::Write> GlobalAttributes<W> for Body<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}



