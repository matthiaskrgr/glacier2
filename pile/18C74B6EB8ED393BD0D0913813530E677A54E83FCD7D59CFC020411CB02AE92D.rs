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
//~| ERROR demangling-alt(<foreign_types::Check<foreign_types::ForeignType>>)
impl Check<ForeignType> {
    #[rustc_symbol_name]
    //[v0]~^ ERROR symbol-name
    //[v0]~| ERROR demangling
    //[v0]~| ERROR demangling-alt
    fn method(&self) {}
}

fn main() {}
