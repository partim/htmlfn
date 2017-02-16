
use std::io;


//------------ PhrasingContent -----------------------------------------------

pub struct PhrasingContent<W: io::Write>(W);

impl<W: io::Write> PhrasingContent<W> {
    pub fn new(w: W) -> Self {
        PhrasingContent(w)
    }

    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: io::Write> PhrasingContent<W> {
    // a with only phrasing content
    // abbr
    // audio
    // b
    // bdo
    // bdr
    // button
    // canvas
    // cite
    // code
    // datalist
    // del with only phrasing content
    // dfn
    // em
    // embed
    // i
    // iframe
    // img
    // ins with only phrasing content
    // input
    // kbd
    // label
    // link with itemprop attribute
    // map with only phrasing content
    // mark
    // math
    // meta with itemprop attribute
    // meter
    // noscript
    // object
    // output
    // progress
    // q
    // ruby
    // samp
    // script
    // select
    // small
    // span
    // strong
    // sub
    // sup
    // svg
    // textarea
    // time
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

