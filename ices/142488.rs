trait Parser {
    fn map(&self) {}
}
impl<A: Parser> Parser for (A,) {}
fn Parser() {
    (Ok(0),).map()
}

fn main() {}
