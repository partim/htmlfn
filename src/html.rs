use crate::core::{
    Attributes, AttributeValue, Element, EmptyElement, Content, Raw, Text,
    TextElement, Tokens,
};

//------------ doctype -------------------------------------------------------

pub fn doctype() -> impl Content {
    Raw::new("<!DOCTYPE html>")
}

//------------ Elements ------------------------------------------------------

macro_rules! standard {
    ( $hx:ident ) => {
        pub fn $hx(content: impl Content) -> impl Content {
            Element::new(stringify!($hx), (), content)
        }

        pub mod $hx {
            use super::*;

            pub fn attrs(
                attrs: impl Attributes,
                content: impl Content
            ) -> impl Content {
                Element::new(stringify!($hx), attrs, content)
            }

            pub fn class<'a>(
                class: impl super::Tokens<'a>,
                content: impl Content
            ) -> impl Content {
                Element::new(stringify!($hx), attr::class(class), content)
            }

            pub fn id(
                id: impl AttributeValue, content: impl Content
            ) -> impl Content {
                Element::new(stringify!($hx), attr::id(id), content)
            }
        }
    }
}

//--- a

pub fn a(href: impl AttributeValue, content: impl Content) -> impl Content {
    Element::new("a", self::attr::href(href), content)
}

pub mod a {
    use super::*;

    pub fn attrs(
        attrs: impl Attributes,
        content: impl Content
    ) -> impl Content {
        Element::new("a", attrs, content)
    }

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        href: impl super::AttributeValue,
        content: impl Content
    ) -> impl Content {
        Element::new("a", (attr::class(class), attr::href(href)), content)
    }
}

//--- body

pub fn body(content: impl Content) -> impl Content {
    Element::new("body", (), content)
}

//--- button

pub fn button(
    button_type: impl AttributeValue,
    attrs: impl Attributes,
    content: impl Content
) -> impl Content {
    Element::new("button", (attr::type_(button_type), attrs), content)
}

//--- div

pub fn div(content: impl Content) -> impl Content {
    Element::new("div", (), content)
}

pub mod div {
    use super::*;

    pub fn attrs(
        attrs: impl Attributes,
        content: impl Content
    ) -> impl Content {
        Element::new("div", attrs, content)
    }

    pub fn id_class<'a>(
        id: impl AttributeValue,
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("div", (attr::id(id), attr::class(class)), content)
    }

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("div", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("div", attr::id(id), content)
    }
}

//--- dl, dd, dt

standard!(dl);
standard!(dd);
standard!(dt);

//--- footer

pub fn footer(content: impl Content) -> impl Content {
    Element::new("footer", (), content)
}

pub mod footer {
    use super::*;

    pub fn attrs(
        attrs: impl Attributes,
        content: impl Content
    ) -> impl Content {
        Element::new("footer", attrs, content)
    }

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("footer", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("footer", attr::id(id), content)
    }
}

//--- form

pub fn form(attrs: impl Attributes, content: impl Content) -> impl Content {
    Element::new("form", attrs, content)
}

//--- h1

standard!(h1);
standard!(h2);
standard!(h3);
standard!(h4);
standard!(h5);
standard!(h6);

//--- head

pub fn head(content: impl Content) -> impl Content {
    Element::new("head", (), content)
}

//--- header

pub fn header(content: impl Content) -> impl Content {
    Element::new("header", (), content)
}

pub mod header {
    use super::*;

    pub fn attrs(
        attrs: impl Attributes,
        content: impl Content
    ) -> impl Content {
        Element::new("header", attrs, content)
    }

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("header", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("header", attr::id(id), content)
    }
}

//--- html

pub fn html(
    lang: impl AttributeValue,
    head: impl Content,
    body: impl Content
) -> impl Content {
    Element::new("html", attr::lang(lang), (head, body))
}

//--- img

pub fn img(src: impl AttributeValue, alt: impl AttributeValue) -> impl Content {
    EmptyElement::new("img", (attr::src(src), attr::alt(alt)))
}

pub mod img{
    use super::*;

    pub fn attrs(attrs: impl Attributes) -> impl Content {
        EmptyElement::new("img", attrs)
    }
}

//--- li

pub fn li(content: impl Content) -> impl Content {
    Element::new("li", (), content)
}

pub mod li {
    use super::*;

    pub fn attrs(
        attrs: impl Attributes,
        content: impl Content
    ) -> impl Content {
        Element::new("li", attrs, content)
    }

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("li", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("li", attr::id(id), content)
    }
}

//--- link

pub mod link {
    use crate::core::{Attr, Attributes, AttributeValue, Content, EmptyElement};

    pub fn link(attrs: impl Attributes) -> impl Content {
        EmptyElement::new("link", attrs)
    }

    pub fn stylesheet(href: impl AttributeValue) -> impl Content {
        link((
            Attr::new("rel", "stylesheet"),
            Attr::new("href", href),
        ))
    }
}

//--- main

pub fn main(content: impl Content) -> impl Content {
    Element::new("main", (), content)
}

pub mod main {
    use super::*;

    pub fn attrs(
        attrs: impl Attributes,
        content: impl Content
    ) -> impl Content {
        Element::new("main", attrs, content)
    }

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("main", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("main", attr::id(id), content)
    }
}

//--- meta

pub mod meta {
    use crate::core::{Attr, AttributeValue, Content, EmptyElement};

    pub fn charset(charset: impl AttributeValue) -> impl Content {
        EmptyElement::new("meta",
            Attr::new("charset", charset),
        )
    }

    pub fn name(
        name: &'static str, content: impl AttributeValue
    ) -> impl Content {
        EmptyElement::new("meta",
            (
                Attr::new("name", name),
                Attr::new("content", content)
            )
        )
    }

    pub fn utf8() -> impl Content {
        charset("utf-8")
    }

    pub fn viewport(content: impl AttributeValue) -> impl Content {
        name("viewport", content)
    }
}

//--- nav

pub fn nav(content: impl Content) -> impl Content {
    Element::new("nav", (), content)
}

pub mod nav {
    use super::*;

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("nav", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("nav", attr::id(id), content)
    }
}

//--- p

pub fn p(content: impl Content) -> impl Content {
    Element::new("p", (), content)
}

//--- span

pub fn span(
    attrs: impl Attributes,
    content: impl Content
) -> impl Content {
    Element::new("span", attrs, content)
}

pub mod span {
    use super::*;

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("span", attr::class(class), content)
    }
}

//--- table

standard!(table);

//--- td

standard!(td);

//--- title

pub fn title(title: impl Text) -> impl Content {
    TextElement::new("title", (), title)
}

//--- tr

standard!(tr);

//--- tt

pub fn tt(
    content: impl Content
) -> impl Content {
    Element::new("tt", (), content)
}

//--- ul

pub fn ul(content: impl Content) -> impl Content {
    Element::new("ul", (), content)
}

pub mod ul {
    use super::*;

    pub fn class<'a>(
        class: impl super::Tokens<'a>,
        content: impl Content
    ) -> impl Content {
        Element::new("ul", attr::class(class), content)
    }

    pub fn id(id: impl AttributeValue, content: impl Content) -> impl Content {
        Element::new("ul", attr::id(id), content)
    }
}


//------------ Attributes ----------------------------------------------------

pub mod attr {
    use crate::core::{
        Attr, AttributeName, Attributes, AttributeValue, Display, Target,
        Tokens,
    };
    use crate::escape;

    pub fn alt(value: impl AttributeValue) -> impl Attributes {
        Attr::new("alt", value)
    }

    pub fn aria(
        key: impl AttributeName, value: impl AttributeValue
    ) -> impl Attributes {
        Attr::new(("aria-", key), value)
    }

    pub fn class<'a>(value: impl Tokens<'a>) -> impl Attributes {
        Attr::new("class", WsTokens(value))
    }

    pub fn data(
        key: impl AttributeName, value: impl AttributeValue
    ) -> impl Attributes {
        Attr::new(("data-", key), value)
    }

    pub fn href(id: impl AttributeValue) -> impl Attributes {
        Attr::new("href", id)
    }

    pub fn id(id: impl AttributeValue) -> impl Attributes {
        Attr::new("id", id)
    }

    pub fn lang(lang: impl AttributeValue) -> impl Attributes {
        Attr::new("lang", lang)
    }

    pub fn src(value: impl AttributeValue) -> impl Attributes {
        Attr::new("src", value)
    }

    pub fn type_(value: impl AttributeValue) -> impl Attributes {
        Attr::new("type", value)
    }

    pub fn width(value: u64) -> impl Attributes {
        Attr::new("src", Display::new(value))
    }




    struct WsTokens<Value>(Value);

    impl<'a, Value: Tokens<'a>> AttributeValue for WsTokens<Value> {
        fn render_attr_value(self, target: &mut Target) {
            let mut iter = self.0.iter_tokens();
            match iter.next() {
                Some(item) => escape::render_attr(item, target),
                None => return,
            }
            for item in iter {
                target.append_slice(b" ");
                escape::render_attr(item, target);
            }
        }
    }

}

