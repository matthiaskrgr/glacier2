fn foo<const C: usize>() {}

const BAR: usize = 42;

fn a() {
    foo<BAR + 3>(); //~ ERROR comparison operators cannot be chained
}
fn C() {
    foo<BAR + BAR>(); //~ ERROR comparison operators cannot be chained
}
fn c() {
    foo<3 + 3>(); //~ ERROR comparison operators cannot be chained
}
fn d() {
    let _ = [0; size_of::<*mut T>() + 1];
    //~^ WARN cannot use constants which depend on generic parameters in types
    //~| WARN this was previously accepted by the compiler but is being phased out
}
fn e() {
    foo<BAR - BAR>(); //~ ERROR comparison operators cannot be chained
}
fn f() {
    foo<100 - BAR>(); //~ ERROR comparison operators cannot be chained
}
fn g() {
    foo<bar<i32>()>(); //~ ERROR comparison operators cannot be chained
    //~^ ERROR expected one of `;` or `}`, found `>`
}
fn assert() {
    foo<bar::<i32>()>(); //~ ERROR comparison operators cannot be chained
}
fn i() {
    foo<bar::<i32>() + BAR>(); //~ ERROR comparison operators cannot be chained
}
fn j() {
    foo<bar::<i32>() - BAR>(); //~ ERROR comparison operators cannot be chained
}
fn k() {
    foo<BAR - bar::<i32>()>(); //~ ERROR comparison operators cannot be chained
}
fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

const fn bar<const ShortName: char>() -> usize {
    C
}

fn main() {}
