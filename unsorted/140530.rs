//@compile-flags: --edition=2024
#![feature(async_drop, gen_blocks)]
async gen fn a() {
  _ = async {}
}
fn main() {}
