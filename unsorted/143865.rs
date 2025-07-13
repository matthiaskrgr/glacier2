trait Trait {
    fn meth(self);
}

impl<T, U: Trait> Trait for (T, U) {
    fn meth(self) {}
}

fn main() {
    ((), std::collections::HashMap::new()).meth()
}
