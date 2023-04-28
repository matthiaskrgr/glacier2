// check-pass
// edition:2018

struct Foo<'a>(&'a ());
impl<'a> Foo<'a> {
    fn main() {}
}

type Alias = Foo<'static>;
impl Alias {
    async fn bar<'a>(self: &Alias, arg: &'a ()) -> &() { arg }
}

fn main() {}
