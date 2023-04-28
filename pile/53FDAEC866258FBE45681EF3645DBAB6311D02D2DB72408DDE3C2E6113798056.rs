// needs-asm-support
// ignore-nvptx64
// ignore-spirv
// ignore-wasm32
// Make sure rustc doesn't ICE on asm! when output type is !.

use std::arch::asm;

fn hmm() -> ! {
    let x;
    hmm();
    x
}

fn main() {
    hmm();
}
