// run-pass
fn main() {
    &0u8 as *const u8 as *const [u8; 1];
    &[0u8] as *const [u8; 1] as *const [u8];
}
