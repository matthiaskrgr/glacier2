#!/bin/bash

cat > out.rs <<'EOF'

#![deny(broken_intra_doc_links)]
#![feature(no_core, lang_items)]
#![no_core]
#![crate_type = "rlib"]


// @has prim_methods_local/index.html
// @has - '//*[@id="main"]//a[@href="{{channel}}/std/primitive.char.html"]' 'char'
// @has - '//*[@id="main"]//a[@href="{{channel}}/std/primitive.char.html#method.len_utf8"]' 'char::len_utf8'

//! A [`char`] and its [`char::len_utf8`].

#[lang = "char"]
impl char {
    pub fn len_utf8(self) -> usize {
        42
    }
}

#[lang = "sized"]
pub trait Sized {}

#[lang = "clone"]
pub trait Clone: Sized {}

#[lang = "copy"]
pub trait Copy: Clone {}

EOF

rustdoc --edition=2021 out.rs
