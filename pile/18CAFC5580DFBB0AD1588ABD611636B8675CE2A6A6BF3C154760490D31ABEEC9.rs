trait Foo<'a, const N: <T as Foo>::Assoc<3>> {
    type Assoc<const N: <Foo as Foo>::Assoc<3>, const N: <T as Assoc>::Assoc<3>>;
}

trait Foo<'a, const N: <T as T>::Assoc<3>> {
    type Foo<const N: <T as Foo>::Assoc<3>, const N: <T as Foo>::Assoc<3>>;
}

fn main() {}
