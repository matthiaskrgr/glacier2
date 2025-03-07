//@compile-flags: -Zincremental-verify-ich=yes -Cincremental=<dir> -Cdebuginfo=2 -Clink-dead-code=true -Zvalidate-mir --edition=2024
pub unsafe auto trait DynSend {}

macro_rules! impls_dyn_send_neg {
    ($([$t1: ty $(where $($generics1: tt)*)?])*) => {
        $(impl$(<$($generics1)*>)? !DynSend for $t1 {})*
    };
}

impls_dyn_send_neg!(
    [*mut T where T: ?Sized]
);
