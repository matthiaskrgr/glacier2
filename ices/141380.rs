trait Trait<T> {}
type Alias<P: Trait> = impl Trait<U>;

pub enum UninhabitedVariants {
    Tuple(Alias),
}

fn uwu(x: UninhabitedVariants) {
    match x {}
}
