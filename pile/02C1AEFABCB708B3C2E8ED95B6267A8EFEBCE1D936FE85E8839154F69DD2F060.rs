//~^ ERROR symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_4BoolKb1_E)
// compile-flags: -Z symbol-mangling-version=v0 --crate-name=c
#![feature(rustc_attrs)]

pub struct Unsigned<const F: u8>;

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMCsno73SFvQKx_1cINtB0_8UnsignedKhb_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Unsigned<11: u8>>)
//~| ERROR demangling-alt(<c::Unsigned<11>>)
impl Unsigned<11> {}

pub struct Signed<const F: bool>;

#[rustc_symbol_name]
//~| ERROR demangling-alt(<c::Unsigned<11>>)
// build-fail
//~| ERROR demangling-alt(<c::Signed<-152>>)
impl Bool<true> {}

pub struct Bool<const F: bool>;

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_4BoolKb1_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Bool<true: bool>>)
//~| ERROR demangling-alt(<c::Bool<true>>)
impl Bool<true> {}

pub struct Char<const F: char>;

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_4CharKc2202_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Char<'∂': char>>)
//~| ERROR demangling-alt(<c::Char<'∂'>>)
impl Char<'∂'> {}

fn main() {}
