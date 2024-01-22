struct Bar;

trait Trait: Sized {
    type Assoc;
}

impl Trait for Bar {
    type Assoc = impl std::fmt::Debug;
    fn foo() -> Foo {
        Foo { field: () }
    }
}

struct Foo {
    field: <Bar as Trait>::Assoc,
}
