extern crate xml;

pub mod common;
pub mod flow;
pub mod format;
pub mod html;
pub mod metadata;
pub mod phrasing;


//============ Convenience Macros ============================================

macro_rules! html {
    ( $wr:expr => { $item:ident $( $content:tt )* } ) => {{
        html_item!($wr => $item $( $content )*)
    }}
}

macro_rules! html_item {
    ( $wr:expr => ) => {{ Ok($wr) }};

    ( $wr:expr => html ( $( $key:ident=$value:expr ),* )
                       { head ( $( $headkey:ident=$headvalue:expr ),* )
                              { $( $headcontent:tt )* }
                         body ( $( $bodykey:ident=$bodyvalue:expr ),* )
                              { $( $bodycontent:tt )* }
                       } ) => {{
        $crate::html::Html::new($wr)?
            $( .$key($value)? )*
            .head()?
                $( .$headkey($headvalue)? )*
                .content(|c| html_item!(c => $( $headcontent )*))?
            .body()?
                $( .$bodykey($bodyvalue)? )*
                .content(|c| html_item!(c => $( $bodycontent )*))
    }};

    ( $wr:expr => text ( $text:expr ); $( $tail:tt )* ) => {{
        let wr = $wr.text($text)?;
        html_item!(wr => $( $tail )*)
    }};

    ( $wr:expr => call ( $closure:expr ); $( $tail:tt )* ) => {{
        $wr.call($closure)
    }};


    ( $wr:expr => $item:ident ( $( $key:ident=$value:expr ),* );
                  $( $tail:tt )* ) => {{
        let wr = $wr.$item()?
                    $( .$key($value)? )*
                    .done()?;
        html_item!(wr => $( $tail )* )
    }};


    ( $wr:expr => $item:ident ( $( $key:ident=$value:expr ),* )
                             { $( $content:tt )* }
                             $( $tail:tt )* ) => {{
        let wr = $wr.$item()?
                    $( .$key($value)? )*
                    .content(|wr| html_item!(wr => $( $content )*))?;
        html_item!(wr => $( $tail )*)
    }}
}

