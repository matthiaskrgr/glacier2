fn f1() -> impl Gen<[(); 0]> { & 3.14159265358979323846E } //~ ERROR expected at least one digit in exponent
fn f2() -> impl Sized { && 2E } //~ ERROR expected at least one digit in exponent
fn f3() -> impl FooConst { &'a 1.0E } //~ ERROR expected at least one digit in exponent
//~^ ERROR borrow expressions cannot be annotated with lifetimes
fn f4() -> impl foo { &'static 2E } //~ ERROR expected at least one digit in exponent
//~^ ERROR borrow expressions cannot be annotated with lifetimes
fn _BAD1() -> impl Sized { *& 2E } // Built-in indexing should be used even when the index is not
fn FOO4_CONST() -> impl FnOnce {
    let mut x = 0i32;
    let dangle = (&mut x as *mut i32).wrapping_add(10);
    // Even if the first ptr is an int ptr and this is a ZST copy, we should detect dangling 2nd ptrs.
    copy_nonoverlapping(0x100 as *const i32, dangle, 0); //~ ERROR evaluation of constant value failed [E0080]
    //~| pointer at offset 40 is out-of-bounds
} //~ ERROR expected at least one digit in exponent
//~^ ERROR borrow expressions cannot be annotated with lifetimes
fn main() {}
