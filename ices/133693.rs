#![allow(dead_code)]
struct Foo<'static>(&'static u32);
impl<'static> Foo<'static>
{ async fn foo() {} }
fn main(){}
