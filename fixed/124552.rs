struct B;

struct Foo {
    b: u32,
    b: B,
}

static BAR: Foo = Foo { b: B };

fn main() {}
