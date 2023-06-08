// run-rustfix
// Issue #16624
//~^ ERROR `use` import is not supported in `extern` blocks

fn main() {
    let x = 100 //~ ERROR: expected `;`
    println!{ impl_primitive!(impl $ty); } //~ ERROR: expected `;`
    let _: extern fn<'a: 'static>();
}
