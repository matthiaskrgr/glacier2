//@compile-flags: -Zincremental-verify-ich=yes -Cincremental=<dir> -Cdebuginfo=2 -Clink-dead-code=true -Zvalidate-mir --edition=2024
trait Q {
    const ASSOC: usize;
}

impl<'a, W: ?Sized> Q for [u8; N] {}

pub fn test() -> [u8; <[u8; 13] as Q>::ASSOC] {
    todo!()
}
