trait Mirror {
    type Assoc;
}

struct Foo;

fn main() {
    <Foo as Mirror>::Assoc::new();
}
