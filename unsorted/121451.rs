use std::mem;

macro_rules! define_reify_functions {
    ($(
        fn $name:ident $(<$($param:ident),*>)?
            for $(extern $abi:tt)? fn($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty;
    )+) => {
        $(pub const fn $name<
            $($($param,)*)?
            F: Fn($($arg_ty),*) -> $ret_ty + Copy
        >(f: F) -> $(extern $abi)? fn($($arg_ty),*) -> $ret_ty {


            assert!(mem::size_of::<F>() == 0, );

            $(extern $abi)? fn wrapper<
                $($($param,)*)?
                F: Fn($($arg_ty),*) -> $ret_ty + Copy
            >($($arg: $arg_ty),*) -> $ret_ty {
                let f = unsafe {


                    mem::MaybeUninit::<F>::uninit().assume_init()
                };
                f($($arg),*)
            }
            let _f_proof = f;
            wrapper::<
                $($($param,)*)?
                F
            >
        })+
    }
}

define_reify_functions! {
    fn _reify_to_extern_c_fn_unary<A, R> for extern "C" fn(arg: A) -> R;
    fn reify_to_extern_c_fn_hrt_bridge<R> for extern "C" fn(bridge: super::BridgeConfig<'_>) -> R;
}
