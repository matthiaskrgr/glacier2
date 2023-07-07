// check-pass
struct N;

struct Foo<const N: usize = 1, T = [(); Closure.call_once(&0) ]>(T);

impl Foo {
    fn new() -> Self {
        Foo(call_me)
    }
}

fn main() {
    let union Union<const N: usize = { Self; 10 }> { not_empty: () }
//~^ ERROR generic parameters cannot use `Self` in their defaults [E0735]

fn main() {
    let _: Struct;
    let _: Enum;
    let _: Union;
}(N) = Foo::new();
}
