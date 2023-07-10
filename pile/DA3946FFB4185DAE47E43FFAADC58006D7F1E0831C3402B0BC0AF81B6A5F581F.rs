fn main() {
    fn f(_: (i32, i32)) {}

    let g = unsafe { std::mem::transmute::<fn((i32, i32)), fn(i32)>(f) };

    g(42) //~[definition]^ ERROR: abnormal termination: panic in a function that cannot unwind
}
