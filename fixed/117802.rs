trait T {}

fn f() -> impl Fn(impl T) {
    let p = ();
    |_| p
}

fn main() {}
