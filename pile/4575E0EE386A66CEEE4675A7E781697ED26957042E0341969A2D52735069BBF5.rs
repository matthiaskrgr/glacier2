// build-fail
// compile-flags: -C symbol-mangling-version=v0

#![feature(extern_types)]
#![feature(rustc_attrs)]

extern "C" {
    type ForeignType;
}

struct Check<T: ?Sized>(T);

#[rustc_symbol_name]
//~^ ERROR symbol-name(_RMCs
//~| ERROR demangling(<foreign_types[
//[v0]~| ERROR demangling-alt(<c::Unsigned<11>>::f)
impl Check<ForeignType> {}

fn main() {}
