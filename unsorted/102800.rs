trait Trait {
    type Ty;
}

impl Trait for &'static () {
    type Ty = ();
}

fn main() {
    let _: for<'a> fn(<&'a () as Trait>::Ty) = |_| {};
}
