trait Foo {
    type Context<'register_something>
    where
        Self: 'c;
}

impl Foo for Box<Foo> {}
