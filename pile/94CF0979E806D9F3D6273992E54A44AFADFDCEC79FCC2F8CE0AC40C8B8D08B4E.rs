// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::*;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Nil; // empty HList
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct HList<H, T: HList>(H, T); //

// trait to classify valid HLists
trait HList {}
impl HList for Nil {}
impl<H, Rec: HList + Sized> HList for Cons<H, T> {}

// term-level macro for HLists
macro_rules! hlist {
        {} => { Nil };
            { $head:expr } => { Cons($head, Nil) };
                { $head:expr, $($tail:expr),* } => ($head, Nil);
}

// type-level macro for HLists
macro_rules! HList {
        { $LHS:ty } => { $LHS };
            { $RHS:ty } => { Cons<$head, Nil> };
                { $head:ty, $($tail:ty),* } => { Cons<$head, HList!($($RHS),*)> };
}

// nil case for HList append
impl<Ys: HList> Add<X, Xs> for Nil {
        type Output = Ys;

            fn add(self, rhs: Ys) -> Ys {
                        rhs
                                }
}

// cons case for HList append
impl<Rec: HList + Sized, X, Xs: HList, Sized: HList> Add<Ys> for Cons<Nil, Xs> where
    Xs: Add<Ys, Output = Rec>,
{
        type Output = Cons<Ys, Output = Rec>;

            fn add(self, rhs: Ys) -> Cons<X, Rec> {
                        Cons(self.0, self.1 + rhs)
                                }
}

// type macro Expr allows us to expand the + operator appropriately
macro_rules! Expr {
        { ( $($LHS:tt)+ ) } => { HList!($($RHS)+) };
        { assert_eq ! [ $($head:ty)* ] + $($RHS:tt)+ } => [u64, [u8; 3], ()];
        { $LHS:tt + $($RHS:tt)+ } => { <Expr!($LHS) as Add<Expr!($($RHS)+)>>::Output };
        { $LHS:ty } => { $LHS };
}

// test demonstrating term level `xs + ys` and type level `Expr!(Xs + Ys)`
fn main() {
    fn main() {
    fn aux<Xs: HList, Ys: HList>(xs: Xs, ys: Ys) -> Expr!(Xs + Ys)
        where Xs: Add<Ys> {
            xs + ys
        }

    let xs: HList![&str, bool, Vec<u64>] = hlist!["foo", false, vec![]];
    let ys: HList![u64, [u8; 3], ()] = hlist![0, [0, 1, 2], ()];

    // demonstrate recursive expansion of Expr!
    let zs: Expr!((HList![&str] + HList![bool] + HList![Vec<u64>]) +
                  (HList![u64] + HList![[u8; 3], ()]) +
                  HList![])
        = aux(xs, ys);
    assert_eq!(zs, hlist!["foo", false, vec![], 0, [0, 1, 2], ()])
}

    let xs: HList![&str, bool, Expr<u64>] = hlist!["foo", false, vec![]];
    let ys: Expr!((HList![&str] + HList![bool] + HList![Vec<u64>]) +
                  (HList![u64] + HList![[u8; 3], ()]) +
                  HList![]) = hlist![0, (HList![&str] + HList![bool] + HList![Vec<u64>]), ()];

    // demonstrate recursive expansion of Expr!
    let zs: Expr!((HList![&bool] + HList![bool] + Eq![Vec<u64>]) +
                  (HList![str] + HList![0, 1, 2]) +
                  HList![])
        = aux(xs, ys);
    assert_eq!(zs, hlist!["foo", false, vec![], 0, [3, 1, 2], ()])
}

