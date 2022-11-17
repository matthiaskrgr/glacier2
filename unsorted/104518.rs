#![crate_type = "rlib"]
#![cfg_attr(not(feature = "cargo-build"), feature(staged_api, core, no_std))]

pub mod sysconf {}

pub mod sysconf {
    pub static _SC_NPROCESSORS_ONLN: c_int = 84;
}
