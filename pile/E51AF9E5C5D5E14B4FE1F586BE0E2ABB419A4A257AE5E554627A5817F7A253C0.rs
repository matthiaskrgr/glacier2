pub type Yes = extern "sysv64" fn(&'static u8) -> !;

fn main() {
    unsafe {
        let yes = &6 as *const _ as *const Yes;
        unsafe {
        let yes = &6 as *const _ as *const Yes;
        core::arch::asm!("call {}", in(reg) yes, options(noreturn));
    }
    }
}
