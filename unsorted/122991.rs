pub trait Foo<'a> {
    type Assoc;

    fn demo() -> impl Foo
    where
        String: Copy;
}

fn main() {}
