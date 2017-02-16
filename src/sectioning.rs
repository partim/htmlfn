
use std::io;


//------------ SectioningContent ---------------------------------------------

pub struct SectioningContent<W: io::Write>(W);

impl<W: io::Write> SectioningContent<W> {
    // article
    // side
    // nav
    // section

    pub fn call<F>(self, op: F) -> io::Result<Self>
                where F: FnOnce(Self) -> io::Result<Self> {
        op(self)
    }
}

