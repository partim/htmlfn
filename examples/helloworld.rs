#[macro_use] extern crate htmlfn;

use std::io;
use htmlfn::core::Content;


fn scaffolding<T: Content, C: Content>(title: T, core: C) -> impl Content {
    html!(
        @html {
            @head {
                @title { title }
            }
            @body() {
                @div(class="header") {
                    @ul() {
                        @li() {
                            @a(href="#") { "Home" }
                        }
                        @li() {
                            @a(href="#") { "About" }
                        }
                    }
                }
                @div(class="core") { core }
                @div(class="footer") {
                    @p() { "Copyright notice and whatnot" }
                }
            }
        }
    )
}


fn index() -> impl Content {
    scaffolding("Hello World!", "Hello World!")
}


fn main() {
    let target = io::stdout();
    let mut target = target.lock();
    index().write(&mut target).unwrap();
}
