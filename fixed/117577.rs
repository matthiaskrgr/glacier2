struct Ty1;
struct Ty2;
struct Ty3;

pub trait TraitAssoc {
    type Assoc;
}

impl TraitAssoc for Ty1 {
    type Assoc = Ty2;
}

trait MyTrait<T: TraitAssoc<Assoc = Self>> {
    type Out;
}

impl<T: TraitAssoc<Assoc = Ty2>> MyTrait<T> for Ty2 {
    type Out = Ty3;
}

fn main() {
    let _: <_ as MyTrait<Ty1>>::Out;
}
