
use std::io;
use super::common::GlobalAttributes;
//use super::format::{AttributeValue, PCData};
use super::phrasing::PhrasingContent;


//------------ FlowContent ---------------------------------------------------

pub struct FlowContent<W: io::Write>(W);

impl<W: io::Write> FlowContent<W> {
    pub fn new(w: W) -> Self {
        FlowContent(w)
    }

    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: io::Write> FlowContent<W> {
    // a
    // abbr
    // address
    // article
    // aside
    // audio
    // b
    // bdo
    // bdi
    // blockquote
    // br
    // button
    // canvas
    // cite
    // code
    // data
    // datalist
    // del
    // details
    // dfn
    // div
    // dl
    // em
    // embed
    // fieldset
    // figure
    // footer
    // form
    pub fn h1(self) -> io::Result<H1<W>> { H1::new(self) }
    // h2
    // h3
    // h4
    // h5
    // h6
    // header
    // hgroup
    // hr
    // i
    // iframe
    // img
    // input
    // ins
    // kbd
    // label
    // link with itemprop attribute
    // main
    // map
    // mark
    // math
    // menu
    // meter
    // nav
    // noscript
    // object
    // ol
    // output
    pub fn p(self) -> io::Result<P<W>> { P::new(self) }
    // pre
    // progress
    // q
    // ruby
    // s
    // smap
    // script
    // section
    // select
    // small
    // span
    // strong
    // style with scoped attribute
    // sub
    // sup
    // svg
    // table
    // template
    // textarea
    // time
    pub fn ul(self) -> io::Result<Ul<W>> { Ul::new(self) }
    // var
    // video
    // wbr

    pub fn text<T: AsRef<str>>(mut self, text: T) -> io::Result<Self> {
        write!(&mut self.0, "{}", text.as_ref())?;
        Ok(self)
    }

    pub fn call<F>(self, op: F) -> io::Result<Self>
                where F: FnOnce(Self) -> io::Result<Self> {
        op(self)
    }
}


//------------ LiContent -----------------------------------------------------

pub struct LiContent<W: io::Write>(W);

impl<W: io::Write> LiContent<W> {
    pub fn li(self) -> io::Result<Li<W>> {
        Li::new(self)
    }

    pub fn call<F>(self, op: F) -> io::Result<Self>
                where F: FnOnce(Self) -> io::Result<Self> {
        op(self)
    }
}


//------------ H1 ------------------------------------------------------------

pub struct H1<W: io::Write>(W);

impl<W: io::Write> H1<W> {
    fn new(mut c: FlowContent<W>) -> io::Result<Self> {
        c.0.write_all(b"<h1")?;
        Ok(H1(c.0))
    }

    pub fn content<F>(mut self, op: F) -> io::Result<FlowContent<W>>
                   where F: FnOnce(PhrasingContent<W>)
                                   -> io::Result<PhrasingContent<W>> {
        self.0.write_all(b">")?;
        let mut wr = op(PhrasingContent::new(self.0))?.into_inner();
        wr.write_all(b"</h1>")?;
        Ok(FlowContent(wr))
    }
}

impl<W: io::Write> GlobalAttributes<W> for H1<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}


//------------ Li ------------------------------------------------------------

pub struct Li<W: io::Write>(W);

impl<W: io::Write> Li<W> {
    pub fn new(mut wr: LiContent<W>) -> io::Result<Self> {
        write!(&mut wr.0, "<li")?;
        Ok(Li(wr.0))
    }

    pub fn value<V: AsRef<str>>(mut self, value: V) -> io::Result<Self> {
        write!(&mut self.0, "value=\"{}\"", value.as_ref())?;
        Ok(self)
    }

    pub fn content<F>(mut self, op: F) -> io::Result<LiContent<W>>
                   where F: FnOnce(FlowContent<W>)
                                   -> io::Result<FlowContent<W>> {
        write!(&mut self.0, ">")?;
        let mut w = op(FlowContent(self.0))?.0;
        write!(&mut w, "</li>")?;
        Ok(LiContent(w))
    }
}


//------------ P -------------------------------------------------------------

pub struct P<W: io::Write>(W);

impl<W: io::Write> P<W> {
    fn new(mut wr: FlowContent<W>) -> io::Result<Self> {
        wr.0.write_all(b"<p")?;
        Ok(P(wr.0))
    }

    pub fn content<F>(mut self, op: F) -> io::Result<FlowContent<W>>
                   where F: FnOnce(PhrasingContent<W>)
                                   -> io::Result<PhrasingContent<W>> {
        self.0.write_all(b">")?;
        let mut w = op(PhrasingContent::new(self.0))?.into_inner();
        w.write_all(b"</p>")?;
        Ok(FlowContent(w))
    }
}

impl<W: io::Write> GlobalAttributes<W> for P<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}


//------------ Ul ------------------------------------------------------------

pub struct Ul<W: io::Write>(W);

impl<W: io::Write> Ul<W> {
    pub fn new(mut wr: FlowContent<W>) -> io::Result<Self> {
        write!(&mut wr.0, "<ul")?;
        Ok(Ul(wr.0))
    }

    pub fn content<F>(mut self, op: F) -> io::Result<FlowContent<W>>
                   where F: FnOnce(LiContent<W>) -> io::Result<LiContent<W>> {
        write!(&mut self.0, ">")?;
        let mut w = op(LiContent(self.0))?.0;
        write!(&mut w, "</ul>")?;
        Ok(FlowContent(w))
    }
}

impl<W: io::Write> GlobalAttributes<W> for Ul<W> {
    fn writer_mut(&mut self) -> &mut W {
        &mut self.0
    }
}

