// A slight variation of issue-85071.rs. Here, a method is called instead
//~^ WARNING: unused variable: `x`
// instead of an unreachable expression.

// check-pass

#![warn(unused_variables,unreachable_code)]

enum Foo {}

struct S;
impl S {
    fn f(&self) -> Foo {todo!()}
}

fn main() {
    let s = S;
    let x = s.f();
    //~^ WARNING: unused variable: `x`
    let _y = x;
    //~^ WARNING: unreachable definition
}
