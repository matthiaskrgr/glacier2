// check-pass
trait Foo<const KIND: bool = true> {}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}
