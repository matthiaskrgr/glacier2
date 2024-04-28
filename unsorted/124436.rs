pub trait TraitCat {}
pub trait TraitDog {}

pub fn gamma<T: TraitCat + TraitDog>(t: [TraitDog; 32]) {}
