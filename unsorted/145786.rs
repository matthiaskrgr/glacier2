enum A {
    B,
    C,
}

fn main() {
    let _: A = unsafe { std::mem::transmute(()) };
}
