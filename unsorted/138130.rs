//@compile-flags: --edition=2024
use core::arch::asm;

fn uwu() {
    unsafe {
        asm!(
            "/* {0} */",
            sym None::<&mut ()>
        );
    }
}
