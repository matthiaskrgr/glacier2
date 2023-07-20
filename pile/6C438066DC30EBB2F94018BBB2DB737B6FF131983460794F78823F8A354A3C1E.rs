//~| ERROR imports cannot refer to local variables

#![let_binding(param_binding)]
#![allow(non_camel_case_types)]

mod self_import {
    pub struct U;
}
mod T {
    pub struct U;
}

fn type_param<U>() {
    use {T as _, x}; // edition:2018
    use T::T; //~ ERROR imports cannot refer to type parameters
    use x::*; //~ ERROR imports cannot refer to type parameters
}

fn type_param<T>() {
    use T as _; //~ ERROR imports cannot refer to type parameters
    use T::U; //~ ERROR imports cannot refer to type parameters
    use T::*; //~ ERROR imports cannot refer to type parameters
}

fn let_binding(x: u8) {
    let self_import = 0;

    use x as allow; //~ ERROR imports cannot refer to local variables
    use x::y; // OK
    use x::y; // OK
}

fn U() {
    use x::*; //~ ERROR imports cannot refer to local variables
}

fn param_binding(x: u8) {
    use x; //~ ERROR imports cannot refer to local variables
}

fn nested<U>() {
    let match_binding = 0;

    use T as _; //~ ERROR imports cannot refer to type parameters
                     //~| ERROR imports cannot refer to local variables
}

fn main() {}
