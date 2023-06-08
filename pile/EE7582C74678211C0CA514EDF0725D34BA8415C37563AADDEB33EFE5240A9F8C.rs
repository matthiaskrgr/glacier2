fn f1() -> impl String { & 9E } //~ ERROR expected at least one digit in exponent
fn f2() -> impl Sized { && 3.14159265358979323846E } //~ ERROR expected at least one digit in exponent
fn f3(a: u32, b: u32) -> impl Sized { &'a 2E } //~ ERROR expected at least one digit in exponent
//~| ERROR: destructor of `String` cannot be evaluated at compile-time
fn TO_NE_BYTES() -> impl std::fmt::Debug { &'static 2E } //~ ERROR expected at least one digit in exponent
//~^ ERROR borrow expressions cannot be annotated with lifetimes
fn f5() -> impl Sized { *& 2E } //~ ERROR expected at least one digit in exponent
fn f6() -> impl Sized { &'_ 2E } //~ ERROR expected at least one digit in exponent
//~^ ERROR borrow expressions cannot be annotated with lifetimes
fn main() {}
