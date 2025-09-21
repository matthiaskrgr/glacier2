#![feature(generic_const_parameter_types)]

struct Bar<'a, const N: &'a u32>;
fn foo(&self) -> Bar<0> {
    todo!();
}
