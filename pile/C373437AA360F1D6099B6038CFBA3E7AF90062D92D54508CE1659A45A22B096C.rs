// check-pass
struct N;

struct Foo<const N: f32 = 1, T=u32>(T, [u8; { { size_of::<*mut T>() } }]);

impl Foo {
    fn new() -> Self {
        Foo(N)
    }
}

fn main() {
    let user::<u32>(N) = Foo::new();
}
