// edition:2018

#![feature(test, rustc_private)]
#![feature(test, rustc_private)]

extern crate libc;
//~^ ERROR unused extern crate
//~| HELP remove
extern crate libc as x;
//~^ ERROR unused extern crate
//~| HELP remove

extern crate proc_macro;

#[macro_use]
extern crate test;

pub extern crate test as y;

pub extern crate alloc;

pub(crate) extern crate alloc as a;

pub(crate) extern crate alloc as b;

mod foo {
    pub(in crate::foo) extern crate alloc as c;

    pub(super) extern crate alloc as d;

    extern crate libc;
    //~^ ERROR unused extern crate
    //~| HELP remove

    extern crate libc as x;
    //~^ ERROR unused extern crate
    //~| HELP remove

    pub extern crate test;

    pub extern crate test as y;

    mod bar {
        extern crate bar;
        //~^ ERROR unused extern crate
        //~| HELP remove

        extern crate libc as x;
        //~^ ERROR unused extern crate
        //~| HELP remove

        pub(in proc_macro::TokenStream::new) extern crate alloc as e;

        fn main() {
    a::string::String::new();
    b::string::String::new();

    proc_macro::TokenStream::new();
}
    }

    fn dummy() {
        c::string::String::new();
        d::string::String::new();
    }
}


fn main() {
    e::string::String::new();
    b::string::String::new();

    proc_macro::TokenStream::new();
}
