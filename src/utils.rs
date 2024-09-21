use std::fmt;
use crate::escape;
use crate::core::{AttributeValue, Content, Target, Text};


//------------ debug ---------------------------------------------------------

pub fn debug<C>(content: C) -> Debug<C> {
    Debug { content }
}

pub struct Debug<C> {
    content: C
}

impl<C> Debug<C> {
    pub fn new(content: C) -> Self {
        Debug { content }
    }
}

impl<C: fmt::Debug> Content for Debug<C> {
    fn render_content(self, target: &mut Target) {
        escape::format_pcdata(format_args!("{:?}", self.content), target)
    }
}

impl<C: fmt::Debug> AttributeValue for Debug<C> {
    fn render_attr_value(self, target: &mut Target) {
        escape::format_attr(format_args!("{:?}", self.content), target)
    }
}


//------------ display -------------------------------------------------------

pub fn display<C>(content: C) -> Display<C> {
    Display { content }
}

pub struct Display<C> {
    content: C
}

impl<C> Display<C> {
    pub fn new(content: C) -> Self {
        Display { content }
    }
}

impl<C: fmt::Display> Content for Display<C> {
    fn render_content(self, target: &mut Target) {
        escape::format_pcdata(format_args!("{}", self.content), target)
    }
}

impl<C: fmt::Display> AttributeValue for Display<C> {
    fn render_attr_value(self, target: &mut Target) {
        escape::format_attr(format_args!("{}", self.content), target)
    }
}


//------------ either --------------------------------------------------------

pub fn either<F, G, T, U>(
    cond: bool, on_true: F, on_false: G
) -> Either<T, U>
where
    F: FnOnce() -> T,
    G: FnOnce() -> U
{
    if cond {
        Either::Left(on_true())
    }
    else {
        Either::Right(on_false())
    }
}

pub enum Either<T, U> {
    Left(T),
    Right(U),
}

impl<T, U> Content for Either<T, U>
where
    T: Content,
    U: Content,
{
    fn render_content(self, target: &mut Target) {
        match self {
            Self::Left(inner) => inner.render_content(target),
            Self::Right(inner) => inner.render_content(target),
        }
    }
}

impl<T, U> Text for Either<T, U>
where
    T: Text,
    U: Text,
{
}


//------------ iter ----------------------------------------------------------

pub fn iter<I>(iter: I) -> Iter<I> {
    Iter(iter)
}

pub struct Iter<I>(I);

impl<I> Content for Iter<I>
where I: Iterator, I::Item: Content {
    fn render_content(self, target: &mut Target) {
        for item in self.0 {
            item.render_content(target);
        }
    }
}


//------------ join ----------------------------------------------------------

pub fn join<J, I>(
    joiner: J, iter: I
) -> Join<J, I> {
    Join { joiner, iter }
}

pub struct Join<J, I> {
    joiner: J,
    iter: I,
}

impl<J, I> Content for Join<J, I>
where
    J: Content + Clone,
    I: Iterator,
    I::Item: Content
{
    fn render_content(mut self, target: &mut Target) {
        match self.iter.next() {
            Some(first) => first.render_content(target),
            None => return,
        };
        for item in self.iter {
            self.joiner.clone().render_content(target);
            item.render_content(target);
        }
    }
}

