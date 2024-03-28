trait Handler {
    fn handle(&self) {}
}

impl<H: Handler, F: Fn() -> H> Handler for F {}

impl<L: Handler> Handler for (L,) {}

fn one() -> impl Handler {
    (one,)
}

fn main() {
    one.handle();
}
