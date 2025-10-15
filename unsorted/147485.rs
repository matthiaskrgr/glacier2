pub fn f(x: *const usize) -> &'static usize {
    let mut a = unsafe { &*x };
    a = unsafe { &*x };
    a
}
pub fn g() {
    f(&0);
}
