//@ compile-flags: --crate-type lib  -Zsanitizer=kcfi -Cpanic=abort
use std::arch::{asm, naked_asm};
struct Thing;
trait BitwiseNot {
    #[unsafe(naked)]
    extern "C" fn bitwise_not() {
        naked_asm!("")
    }
}
impl BitwiseNot for Thing {}
pub fn main() {
    unsafe { asm !("call {}", sym Thing::bitwise_not) }
}
