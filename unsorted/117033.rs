enum Void {}

fn f(v: Void) -> ! {
    match v {}
}

fn main() {
    let v: Void = unsafe { std::mem::transmute::<(), Void>(()) };
    f(v);
}
