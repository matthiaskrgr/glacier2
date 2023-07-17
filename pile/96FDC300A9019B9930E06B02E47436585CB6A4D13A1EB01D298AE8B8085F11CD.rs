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

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"'">>)
impl Str<"'"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling(a::b::Type<str>::
impl Str<"\t\n"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"∂ü">>)
impl Str<"∂ü"> {
        #[rustc_symbol_name]
        //[legacy]~^ ERROR symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz
        //[legacy]~| ERROR demangling(impl1::bar::<impl impl1::foo::Foo>::baz
        //[legacy]~| ERROR demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
         //[v0]~^^^^ ERROR symbol-name(_RNvMNtCs
            //[v0]~| ERROR demangling(<impl1[
            //[v0]~| ERROR demangling-alt(<impl1::foo::Foo>::baz)
        #[rustc_def_path]
        //[legacy]~^ ERROR def-path(bar::<impl foo::Foo>::baz)
           //[v0]~^^ ERROR def-path(bar::<impl foo::Foo>::baz)
        fn baz() { }
    }

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
impl Str<"საჭმელად_გემრიელი_სადილი"> {}

#[rustc_symbol_name]
//~^ ERROR symbol-name
//~| ERROR demangling
//~| ERROR demangling(a::b::Type<u32>::
impl Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜"> {}

fn main() {}
