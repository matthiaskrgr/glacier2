use core::arch::asm;

extern "C" fn test<T>() {}

fn uwu() {
    asm!(
            "/* {0} */",
            sym test::<&mut ()>
        );
}
