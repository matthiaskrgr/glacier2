trait Fun {
    type Assoc;
}

trait Trait: for<'v> Fun<Assoc = &'v ()> {}

impl<F: for<'v> Fun<Assoc = &'v ()>> Trait for F {}

fn main() {}
