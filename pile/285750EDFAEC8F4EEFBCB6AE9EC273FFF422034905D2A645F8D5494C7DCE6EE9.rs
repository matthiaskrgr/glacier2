//~| ERROR demangling-alt(<c::Str<"abc">>)
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
// normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
#![feature(adt_const_params, rustc_attrs)]
#![rustc_symbol_name]

pub struct Str<const S: &'static str>;

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
// normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[rustc_symbol_name]
//~| ERROR demangling
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"'">>)
impl Str<"'"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"\t\n">>)
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

fn main() {}
