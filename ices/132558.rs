// Outcomes of running `cargo run` for different recursion limits:
//
// 20 => panics quickly.
// 30 => panics after 2 minutes.
// default => runs for hours until I kill it.
#![recursion_limit = "20"]

trait S {
    type Child: S;
    fn children(&self) -> Vec<Self::Child>;
}

impl<X: S> S for (X, X) {
    type Child = (X::Child, X::Child);
    fn children(&self) -> Vec<Self::Child> {
        vec![]
    }
}

impl<X: S> S for Option<Box<X>> {
    type Child = X;
    fn children(&self) -> Vec<Self::Child> {
        vec![]
    }
}

type Data = Option<Box<(A, A)>>;
struct A {
    data: Data,
}
impl S for A {
    // When the recursion limit is set to 20, manually expanding the `Data::Child` 
    // type here makes the compilation fail with a "reached the recursion limit" error, 
    // as it should. But with the default recursion limit, the compilation hangs whether
    // I manually expand the type here or not.
    //
    //type Child = (A, A);
    type Child = <Data as S>::Child;
    fn children(&self) -> Vec<Self::Child> {
        self.data.children()
    }
}

// The problem: this function can't be monomorphized when called with an
// argument of type `A`, because it recurses heterogenously on `A::Child =
// (A,A)`, which induces monomorphization at the type `(A,A)`, which recurses on
// `(A,A)::Child = ((A,A),(A,A))`, and so on, ad infinitum.
fn uhoh<T: S>(x: &T) {
    let children = x.children();
    println!("{}", children.len());
    for child in &children {
        uhoh(child);
    }
}

fn main() {
    let a = A { data: None };
    uhoh(&a);
}
