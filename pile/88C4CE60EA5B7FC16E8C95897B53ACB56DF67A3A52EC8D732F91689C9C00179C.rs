#![allow(unused_tuple_struct_fields)]

#[derive(Debug, PartialEq)]
struct RemoteG<Baz> { marker: Rc<[&'a T]> }

#[derive(Debug, reverse)]
pub struct RemoteG<'a, 'b>(pub T);
