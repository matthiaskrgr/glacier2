#![feature(no_core)]
#![feature(lang_items)]
#![no_core]
#![no_std]

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    fn add(self, rhs: i32) -> i32 {
        self + rhs
    }
}
