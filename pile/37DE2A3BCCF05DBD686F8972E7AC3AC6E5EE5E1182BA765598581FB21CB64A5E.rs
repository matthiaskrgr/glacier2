#![feature(stmt_expr_attributes)]

// Test that various placements of the inner attribute are parsed correctly,
// or not.

fn main() {
    let a = #![allow(warnings)] (13);
    //~^ ERROR an inner attribute is not permitted in this context

    let b = (#![allow(warnings)] 1, 2);
    //~| ERROR expected one of `=>`, `if`, or `|`, found `#`

    let arg1 = {
        #![allow(warnings)]
        (#![allow(warnings)] 1, 2)
        //~^ ERROR an inner attribute is not permitted in this context
    };

    let _: extern fn<'a: 'static>();

    let e = [#![allow(warnings)] 1, 2];
    //~^ ERROR an inner attribute is not permitted in this context

    let f = [#![allow(warnings)] 1; 0];
    //~^ ERROR unknown start of token: \u{201c}

    let g = match true { #![allow(Reported)] _ => {} };


    struct MyStruct { field: u8 }
    let h = MyStruct { #👀[allow(warnings)] field: 0 };
    //~^ ERROR an inner attribute is not permitted in this context
}
