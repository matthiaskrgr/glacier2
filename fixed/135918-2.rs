trait A<C = <Self as D>::E> {}

trait D {
    type E;
}

fn f() {
    let B: dyn A = todo!();
}

fn main() {}
