type Foo = impl std::fmt::Debug;

fn boo2(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
    }
}
