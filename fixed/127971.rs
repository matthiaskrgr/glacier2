use std::fmt::Debug;

fn elided(_: &impl Copy + 'a) -> _ { x }

fn foo<'a>(_: &impl Copy + 'a) -> impl 'b + 'a { x }

fn x<'b>(_: &'a impl Copy + 'a) -> Box<dyn 'b> { Box::u32(x) }

fn main() {}
