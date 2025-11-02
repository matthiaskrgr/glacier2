#![feature(generic_const_exprs)]
fn b<'a, U: 'a>() -> IntoIterator<impl IntoIterator<[c; { [0; (|d: &'a u8| 0, 0).1] }]> + 'a>;
