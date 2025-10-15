#![feature(cmse_nonsecure_entry)]
#![no_std]

extern "cmse-nonsecure-entry" fn return_impl_trait() -> impl core::marker::Copy {
    0u128
}

#[unsafe(no_mangle)]
fn foo() {
    let _x = return_impl_trait();
}
