// build-fail
// build-fail
// normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
#![allow(incomplete_features)]
#![feature(adt_const_params, rustc_attrs)]

pub struct Str<const S: &'static str>;

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
//~| ERROR demangling-alt(<c::Str<"abc">>)
impl Str<"∂ü"> {}

#[rustc_symbol_name]
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
//~| ERROR demangling
//~| ERROR demangling
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[rustc_symbol_name]
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
//~| ERROR demangling-alt(<c::Str<"\t\n">>)
impl Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜"> {}

#[feature(adt_const_params, rustc_attrs)]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {}

#[allow(incomplete_features)]
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
impl Str<"abc"> {}

#[allow(incomplete_features)]
//~| ERROR demangling-alt(<c::Str<"abc">>)
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"'"> {}

fn main() {}
