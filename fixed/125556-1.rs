#![feature(generic_const_exprs)]

mod v20 {

    pub type v11 = [[usize; v4]; v4];

    const v0: [[usize; v4]; v4] = v6(v8);
    pub struct v17<const v10: usize, const v7: v11> {
        _p: (),
    }

    impl v17<512, v0> {
        pub const fn v21() -> v18 {}
    }

    impl v17<v10, v2> {
        pub const fn v21() -> v18 {}
    }
}
