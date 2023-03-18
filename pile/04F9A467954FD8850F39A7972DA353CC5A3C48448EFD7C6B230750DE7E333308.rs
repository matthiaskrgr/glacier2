#![feature(link_cfg)]

#[link(name = "C", kind = "static", kind(link_cfg))]
extern "C" {}

#[link(feature = "native_dep_2", kind = "static", cfg(should_not_add))]
extern "C" {}

#[no_mangle]
pub fn rust_dep() {}
