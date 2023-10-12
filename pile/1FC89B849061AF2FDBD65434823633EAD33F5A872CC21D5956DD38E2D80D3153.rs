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
//~^ ERROR symbol-name(_ZN1a1b34Type$LT$$LP$u8$C$u16$C$u32$RP$$GT$
impl Check<ForeignType> {}

fn main() {}
