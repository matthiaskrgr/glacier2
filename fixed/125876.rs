fn main() {
    let _result = &std::ptr::from_ref(num).cast_mut().as_deref();
}
