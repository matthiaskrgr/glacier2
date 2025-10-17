const Z: *const () = (&() as *const ()).wrapping_byte_add(2);
pub fn main() {
    std::hint::black_box(Z);
}
