use core::arch::asm;

struct TrapFrame;

unsafe extern "C" fn _rust_abi_shim1<A, R>(arg: A, f: fn(A) -> R) -> R {
    f(arg)
}

unsafe extern "C" fn asm() {
    extern "Rust" {
        fn interrupt(tf: &mut TrapFrame);
    }
    asm!(
        "
        la   a1, {irq}
        call {shim}
        ",
        shim = sym crate::_rust_abi_shim1::<&mut TrapFrame, ()>,
        irq = sym interrupt,
        options(noreturn)
    )
}
