use crate::core::{Content, Target};


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

