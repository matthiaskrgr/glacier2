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
// ignore-gnu - vectorcall is not supported by GCC: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=89485
//~| ERROR demangling-alt(<foreign_types::Check<foreign_types::ForeignType>>)
impl Check<ForeignType> {}

fn main() {}
