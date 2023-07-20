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
//[v0]~^ ERROR symbol-name
//~| ERROR demangling-alt(<foreign_types::Check<foreign_types::ForeignType>>)
impl Check<ForeignType> {}

fn main() {}