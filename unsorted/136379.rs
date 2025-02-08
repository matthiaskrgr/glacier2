//@compile-flags: --crate-type=lib
#![feature(min_generic_const_args)]
mod m {
    pub struct Uuid(());

    impl Uuid {
        pub fn encode_buffer() -> [u8; Uuid] {
            []
        }
    }
}
