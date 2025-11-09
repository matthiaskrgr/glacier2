#![feature(associated_const_equality)]

trait Widget {
    const WIDTH: usize;
}

struct Box<const WIDTH: usize, T> {
    inner: T,
}

impl<const WIDTH: usize, T> Box<WIDTH, T> {
    fn new(_: T) -> Self
    where
        T: Widget<WIDTH = { WIDTH }>,
    {
        loop {}
    }

    fn empty<const X: usize>() -> Box<X, Empty<{ X }>> {
        Box::new(Empty)
    }
}

struct Empty<const WIDTH: usize>;

impl<const WIDTH: usize> Widget for Empty<WIDTH> {
    const WIDTH: usize = WIDTH;
}

fn main() {}
