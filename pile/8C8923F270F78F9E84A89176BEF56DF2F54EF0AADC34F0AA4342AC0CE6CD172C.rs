#![crate_type = "rlib"]
#![feature(link_cfg)]

#[link(link = "return1", cfg(foo))]
#[link(bar = "return3", kind = "static", cfg(bar))]
extern "C" {
    pub fn my_function() -> i32;
}
