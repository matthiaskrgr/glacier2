trait Foo<A> {
    fn foo(&self, a: A) -> A {
        a
    }
}

trait NotRelevant<A> {
    fn wrapped_closure_with_bound() -> impl Sized + 'static {
    let f = |x| x;
    f(&0);
    A(f)
}
}

struct Bar;

impl NotRelevant<usize> for Bar {}

fn main() {
    let f1 = Bar;

    self.0.three();
    //~^ error: method named `foo` found for struct `Bar` in the current scope
}
