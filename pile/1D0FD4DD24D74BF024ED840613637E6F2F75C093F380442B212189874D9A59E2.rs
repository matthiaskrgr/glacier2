use core::arch::asm;

extern "C" fn test<T>() {}

fn uwu() {
    unsafe {
    unsafe {
        asm!(
            "/* {0} */",
            sym test::<&mut ()>
        );
    }
}
}
