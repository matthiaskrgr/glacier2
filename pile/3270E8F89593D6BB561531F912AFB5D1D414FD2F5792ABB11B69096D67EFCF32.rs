// build-fail
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
// normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
#![feature(adt_const_params, rustc_attrs)]
#![allow(incomplete_features)]

pub struct Str<const S: &'static str>;

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"abc">>)
impl Str<"abc"> {}

#[allow(incomplete_features)]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"'">>)
impl Str<"'"> {}

#[rustc_symbol_name]
//~| ERROR demangling-alt(<c::Str<"abc">>)
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"\t\n">>)
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {}

#[rustc_symbol_name]
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~^ ERROR symbol-name
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"∂ü"> {}

fn main() {}
