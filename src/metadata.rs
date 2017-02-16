
use std::io;
use super::common::GlobalAttributes;
use super::format::{AttributeValue, PCData};


//------------ MetadataContent -----------------------------------------------

pub struct MetadataContent<W: io::Write>(W);

impl<W: io::Write> MetadataContent<W> {
    pub fn new(w: W) -> Self {
        MetadataContent(w)
    }

    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: io::Write> MetadataContent<W> {
    // base
    pub fn link(self) -> io::Result<Link<W>> { Link::new(self) }
    pub fn meta(self) -> io::Result<Meta<W>> { Meta::new(self) }
    // noscript
    // script
    // style
    pub fn title(self) -> io::Result<Title<W>> { Title::new(self) }

    pub fn call<F>(self, op: F) -> io::Result<Self>
                where F: FnOnce(Self) -> io::Result<Self> {
        op(self)
    }
}


//------------ TitleContent --------------------------------------------------

pub struct TitleContent<W: io::Write>(W);

impl<W: io::Write> TitleContent<W> {
    pub fn text<T: PCData>(mut self, text: T) -> io::Result<Self> {
        text.write_all(&mut self.0)?;
        Ok(self)
    }

    pub fn call<F>(self, op: F) -> io::Result<Self>
                where F: FnOnce(Self) -> io::Result<Self> {
        op(self)
    }
}


//------------ Link ----------------------------------------------------------

pub struct Link<W: io::Write>(W);

impl<W: io::Write> Link<W> {
    fn new(mut wr: MetadataContent<W>) -> io::Result<Self> {
        wr.0.write_all(b"<link")?;
        Ok(Link(wr.0))
    }

    // crossorigin

    pub fn href<V: AttributeValue>(mut self, value: V) -> io::Result<Self> {
        self.0.write_all(b" href=\"")?;
        value.write_all(&mut self.0)?;
        self.0.write_all(b"\"")?;
        Ok(self)
    }
    
    // hreflang
    // media

    pub fn rel<V: AttributeValue>(mut self, value: V) -> io::Result<Self> {
        self.0.write_all(b" rel=\"")?;
        value.write_all(&mut self.0)?;
        self.0.write_all(b"\"")?;
        Ok(self)
    }
    
    // sizes
    // target
    // title
    
    pub fn media_type<V: AttributeValue>(mut self, value: V)
                                         -> io::Result<Self> {
        self.0.write_all(b" type=\"")?;
        value.write_all(&mut self.0)?;
        self.0.write_all(b"\"")?;
        Ok(self)
    }

    pub fn done(mut self) -> io::Result<MetadataContent<W>> {
        self.0.write_all(b"/>")?;
        Ok(MetadataContent(self.0))
    }
}

impl<W: io::Write> GlobalAttributes<W> for Link<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}


//------------ Meta ----------------------------------------------------------

pub struct Meta<W: io::Write>(W);

impl<W: io::Write> Meta<W> {
    fn new(mut wr: MetadataContent<W>) -> io::Result<Self> {
        wr.0.write_all(b"<meta")?;
        Ok(Meta(wr.0))
    }

    pub fn charset<V: AttributeValue>(mut self, value: V) -> io::Result<Self> {
        self.0.write_all(b" charset=\"")?;
        value.write_all(&mut self.0)?;
        self.0.write_all(b"\"")?;
        Ok(self)
    }

    pub fn content<V: AttributeValue>(mut self, value: V) -> io::Result<Self> {
        self.0.write_all(b" content=\"")?;
        value.write_all(&mut self.0)?;
        self.0.write_all(b"\"")?;
        Ok(self)
    }

    // http-equiv
    // name

    pub fn done(mut self) -> io::Result<MetadataContent<W>> {
        self.0.write_all(b"/>")?;
        Ok(MetadataContent(self.0))
    }
}

impl<W: io::Write> GlobalAttributes<W> for Meta<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}


//------------ Title ---------------------------------------------------------

pub struct Title<W: io::Write>(W);

impl<W: io::Write> Title<W> {
    pub fn new(mut c: MetadataContent<W>) -> io::Result<Self> {
        c.0.write_all(b"<title")?;
        Ok(Title(c.0))
    }

    pub fn content<F>(mut self, op: F) -> io::Result<MetadataContent<W>>
                   where F: FnOnce(TitleContent<W>)
                                   -> io::Result<TitleContent<W>> {
        self.0.write_all(b">")?;
        let mut c = op(TitleContent(self.0))?.0;
        c.write_all(b"</title>")?;
        Ok(MetadataContent(c))
    }
}


impl<W: io::Write> GlobalAttributes<W> for Title<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}



