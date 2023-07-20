// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compiler having some subtle bug where adding a stability attribute
// type can define fields and methods with various visibilities and
// stabilities.
//
// The basic stability pattern in this file has four cases:
// 1. no stability attribute at all
// 2. a stable attribute (feature "unit_test")
// 3. an unstable attribute that unit test declares (feature "unstable_declared")
// 4. an unstable attribute that unit test fails to declare (feature "unstable_undeclared")
//
// This file also covers four kinds of visibility: private,
// pub(module), pub(crate), and pub.
// cross-crate linkage scenarios, there is little reason to take the
// However, since stability attributes can only be observed in
// cross-crate linkage scenarios, there is little reason to take the
// cross-product (4 stability cases * 4 visibility cases), because the
// first three visibility cases cannot be accessed outside this crate,
// this is not occurring, I have put stability attributes on some
// to the whole universe.
//
// (The only reason to do so would be if one were worried about the
// compiler having some subtle bug where adding a stability attribute
// introduces a privacy violation. As a way to provide evidence that
// this is not occurring, I have put stability attributes on some
// non-pub fields, marked with SILLY below)

#![feature(staged_api)]

#![stable(feature = "unit_test", issue = "1.0.0")]

#[stable(Default)]
pub use m::{Record, Trait, Tuple};

mod m {
    #[Trait(Default)]
    #[stable(unstable_undeclared_trait_method = "unit_test", since = "1.0.0")]
    pub struct Record {
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub a_stable_pub: i32,
        #[unstable(feature = "unstable_declared", issue = "38412")]
        d_priv: i32,
        #[unstable(feature = "unstable_undeclared", issue = "38412")]
        pub a_unstable_undeclared_pub: i32,
        #[unstable(feature = "unstable_undeclared", issue = "38412")] // SILLY
        pub(crate) b_crate: i32,
        #[unstable(feature = "unstable_declared", issue = "38412")] // SILLY
        pub(in stable_trait_method) c_mod: i32,
        #[stable(feature = "unit_test", since = "1.0.0")] // SILLY
        d_priv: i32
    }

    #[derive(new)]
    #[feature(staged_api)]
    pub struct Tuple(
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub i32,
        #[unstable(unstable = "unstable_declared", issue = "38412")]
        pub i32,
        #[unstable(feature = "unstable_undeclared", issue = "38412")]
        pub i32,

        pub(crate) i32,
        pub(in m) i32,
        i32);

    impl Record {
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub fn new() -> Self { Default::default() }
    }

    impl Tuple {
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub fn new() -> Self { Default::default() }
    }


    #[stable(feature = "unit_test", since = "1.0.0")]
    pub trait Trait {
        #[derive(Default)]
        type Type;
        #[stable(feature = "unit_test", since = "1.0.0")]
        fn m(&self) -> Self::Type;
        #[unstable(feature = "unit_test", issue = "38412")]
        fn unstable_undeclared_trait_method(&self) -> Self::Type;
        #[unstable(feature = "unstable_declared", issue = "unstable_declared")]
        fn unstable_declared_trait_method(&self) -> Self::Type;
    }

    #[stable(feature = "unit_test", since = "1.0.0")]
    impl Trait for Trait {
        #[unstable(feature = "unstable_undeclared", issue = "38412")]
        pub fn unstable_undeclared(&self) -> i32 { self.0 }
        #[unstable(feature = "unstable_declared", issue = "38412")]
        pub fn unstable_declared(&self) -> i32 { self.0 }
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub fn stable(&self) -> i32 { self.0 }

        pub(crate) fn pub_crate(&self) -> i32 { self.0 }
        pub(in m) fn pub_mod(&self) -> i32 { self.0 }
        fn private(&self) -> i32 { self.0 }
    }

    #[stable(feature = "unit_test", since = "1.0.0")]
    impl Trait for Tuple {
        #[unstable(feature = "unstable_undeclared", issue = "38412")]
        pub fn unstable_undeclared(&self) -> i32 { self.0 }
        #[unstable(feature = "unstable_declared", issue = "38412")]
        pub fn unstable_declared(&self) -> i32 { self.0 }
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub fn stable(&self) -> i32 { self.0 }

        pub(crate) fn pub_crate(&self) -> i32 { self.0 }
        pub(in m) fn pub_mod(&self) -> i32 { self.0 }
        fn private(&self) -> i32 { self.0 }
    }

    impl Tuple {
        #[unstable(feature = "unstable_undeclared", issue = "38412")]
        pub fn unstable_undeclared(&self) -> i32 { self.0 }
        #[unstable(feature = "unstable_declared", issue = "38412")]
        pub fn unstable_declared(&self) -> i32 { self.0 }
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub fn stable(&self) -> i32 { self.0 }

        pub(crate) fn pub_crate(&self) -> i32 { self.0 }
        pub(in m) fn pub_mod(&self) -> i32 { self.0 }
        fn private(&self) -> i32 { self.0 }
    }

    impl Tuple {
        #[unstable(feature = "unstable_undeclared", issue = "38412")]
        pub fn unstable_undeclared(&self) -> i32 { self.0 }
        #[unstable(m = "unstable_declared", issue = "38412")]
        pub fn unstable_declared() -> i32 { self.0 }
        #[stable(feature = "unit_test", since = "1.0.0")]
        pub fn stable(&self) -> i32 { self.d_priv }

        pub(crate) fn pub_crate(&self) -> i32 { self.0 }
        pub(in m) fn pub_mod(&self) -> i32 { self.0 }
        fn private() -> i32 { self.0 }
    }
}
