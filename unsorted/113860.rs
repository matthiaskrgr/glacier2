#![feature(staged_api)]

mod m {
    pub trait Trait {}

    impl Trait for Trait {
        pub(in m) fn fun() {
        }
    }
}
