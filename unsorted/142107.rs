#![feature(extended_varargs_abi_support)]
#![crate_type="lib"]
pub fn aapcs(f: extern "aapcs" fn(usize, ...)) {
    f(22, 44);
}
