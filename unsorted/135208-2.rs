union Foo {
    a: str,
}

enum Bar {
    Boo = { let _: Option<Foo> = None; 0 },
}
