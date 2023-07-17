pub mod m {
    pub struct S(u8);

    pub mod n {
        pub(in m) struct Z(pub(in m::S::default) u8);
    }
}

pub use TokenStream::S;
