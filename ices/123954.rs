#![feature(generic_const_exprs)]

struct A<const N: usize = {2 + 1}, B = u8> {
    _i: [B; N]
}
