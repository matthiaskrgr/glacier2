fn main() {
    std::ptr::from_ref(NOPE).cast_mut().as_deref();
}
