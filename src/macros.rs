
#[macro_export]
macro_rules! html {
    (
        $( $content:tt )*
    ) => {
        (
            $crate::core::Raw::new("<!DOCTYPE HTML>"),
            (
                elements!( $( $content )* ),
                $crate::core::Raw::new("\n")
            )
        )
    }
}

#[macro_export]
macro_rules! elements {
    (
        @ $tag:ident ( $( $attrs:tt )* ) {
            $( $content:tt )*
        }
        $( $tail:tt )*
    ) => {
        (
            $crate::core::Element::new(
                stringify!($tag),
                html_attrs!( $( $attrs )* ),
                elements!( $( $content )* )
            ),
            elements!( $( $tail )* )
        )
    };

    (
        @ $tag:ident { $( $content:tt )* } $( $tail:tt )*
    ) => {
        (
            $crate::core::Element::new(
                stringify!($tag),
                html_attrs!(),
                elements!( $( $content )* )
            ),
            elements!( $( $tail )* )
        )
    };

    (
        @ $tag:ident ( $( $attrs:tt )* );
        $( $tail:tt )*
    ) => {
        (
            $crate::core::EmptyElement::new(
                stringify!($tag),
                html_attrs!( $( $attrs )* ),
            ),
            elements!( $( $tail )* )
        )
    };

    (
        @ $tag:ident
        $( $tail:tt )*
    ) => {
        (
            $crate::core::EmptyElement::new(
                stringify!($tag),
                html_attrs!(  ),
            ),
            elements!( $( $tail )* )
        )
    };

    (
        { $expr:expr } $( $tail:tt )*
    ) => {
        ($expr, elements!( $( $tail )* ))
    };

    (
        $expr:expr; $( $tail:tt )*
    )=> {
        ($expr, elements!( $( $tail )* ))
    };

    ( $expr:expr ) => { $expr };

    ( ) => { () }
}

#[macro_export]
macro_rules! html_attrs {
    ( $key:ident = $value:expr, $( $tail:tt )* ) => {
        (
            $crate::core::Attr::new(stringify!($key), $value),
            html_attrs!( $( $tail )* )
        )
    };
    ( $key:ident = $value:expr ) => {
        $crate::core::Attr::new(stringify!($key), $value)
    };
    ( $key:expr => $value:expr, $( $tail:tt )* ) => {
        (
            $crate::core::Attr::new($key, $value),
            html_attrs!( $( $tail )* )
        )
    };
    ( $key:expr => $value:expr ) => {
        $crate::core::Attr::new($key, $value)
    };
    ( ) => {
        ()
    }
}

