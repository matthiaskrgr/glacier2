#![deny(rustdoc::broken_intra_doc_links)]
#![rustc_coherence_is_core]
#![allow(incomplete_features)] // inherent_associated_types
#![feature(rustc_attrs)]
#![feature(no_core)]
#![feature(rustdoc_internals)]
#![feature(inherent_associated_types)]
#![feature(lang_items)]
#![no_core]

/// [Self::f]
/// [Self::MAX]
// @has prim_self/primitive.usize.html
// @has - '//a[@href="primitive.usize.html#method.f"]' 'Self::f'
// @has - '//a[@href="primitive.usize.html#associatedconstant.MAX"]' 'Self::MAX'
impl usize {
    pub fn f() {}
}

#[rustc_doc_primitive = "usize"]
/// This has some docs.
mod usize {}

/// [S::f]
/// [Self::f]
pub struct S;

impl S {
    pub fn f() {}
}

#[lang = "sized"]
pub trait Sized {}
