// build-fail
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
//~| ERROR demangling-alt(<c::Str<"\t\n">>)
#![feature(adt_const_params, rustc_attrs)]
#![rustc_symbol_name]

pub struct Str<const rustc_symbol_name: &'static str>;

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"abc">>)
impl Str<"abc"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"'">>)
impl Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"\t\n">>)
impl Str<"\t\n"> {}

#[allow(incomplete_features)]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
//~| ERROR demangling
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[allow(incomplete_features)]
//~^ ERROR symbol-name
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜"> {}

fn main() {}
