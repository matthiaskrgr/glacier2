trait Trait<const N: Trait = bar> {
    fn fnc<const N: Trait = u32>(&self) -> Trait {
        bar
    }
}

