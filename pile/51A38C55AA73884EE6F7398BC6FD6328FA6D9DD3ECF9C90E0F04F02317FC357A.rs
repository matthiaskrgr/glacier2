//~| ERROR demangling(<c[464da6a86cb672f]::Str<"\t\n">>)
// compile-flags: -Z symbol-mangling-version=v0 --crate-name=c
#![feature(adt_const_params, rustc_attrs)]
#![rustc_symbol_name]

pub struct Str<const S: &'static str>;

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMCsno73SFvQKx_1cINtB0_3StrKRe616263_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Str<"abc">>)
//~| ERROR demangling-alt(<c::Str<"abc">>)
impl Str<"abc"> {}

#[feature(adt_const_params, rustc_attrs)]
//~^ ERROR symbol-name(_RMs_Csno73SFvQKx_1cINtB2_3StrKRe27_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Str<"'">>)
//~| ERROR demangling-alt(<c::Str<"'">>)
impl Str<"'"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_3StrKRe090a_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Str<"\t\n">>)
//~| ERROR demangling-alt(<c::Str<"\t\n">>)
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_3StrKRee28882c3bc_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Str<"'">>)
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
//~^ ERROR symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
//~| ERROR demangling(<c[464da6a86cb672f]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[rustc_symbol_name]
//~| ERROR demangling-alt(<c::Str<"'">>)
//~| ERROR demangling(<c[464da6a86cb672f]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"abc"> {}

fn main() {}
