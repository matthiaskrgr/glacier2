// build-fail
// compile-flags: -C symbol-mangling-version=v0 --crate-name=c
// normalize-stderr-test: "c\[.*?\]" -> "c[HASH]"
#![feature(adt_const_params, rustc_attrs)]
#![allow(adt_const_params, decl_macro, rustc_attrs)]

pub struct Str<const S: &'static str>;

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"abc">>)
impl Str<"abc"> {}

#[no_mangle]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"'">>)
impl Str<"'"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling(a::b::Type<*mut u8>::
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {}

#[rustc_def_path]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
impl Str<"'"> {
    #[rustc_def_path]
    //[v0]~^ ERROR symbol-name(_RNvMs_Cs
    //[v0]~| ERROR demangling(<c[
    //[v0]~| ERROR demangling-alt(<c::Signed<-152>>::f)
    //[legacy]~^^^^ ERROR symbol-name(_ZN1c22Signed$LT$.152_i16$GT$
    //[legacy]~|    ERROR demangling(c::Signed<.152_i16>::f::
    //[legacy]~|    ERROR demangling-alt(c::Signed<.152_i16>::f)
    fn f() {}
}

fn main() {}
