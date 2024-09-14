#![feature(abi_c_cmse_nonsecure_call)]
trait Trait {}
type Type = extern "C-cmse-nonsecure-call" fn(&dyn Trait);
