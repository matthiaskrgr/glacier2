#[allow(dead_code)]
struct Inv<'a>(*mut &'a ());

type F1 = for<'a> fn(Inv<'a>);
type F2 = fn(Inv<'static>);

trait Trait {
    type Assoc: PartialEq;
}
impl Trait for F1 {
    type Assoc = i32;
}
impl Trait for F2 {
    type Assoc = i64;
}

#[derive(PartialEq)]
struct InvTy<T: Trait>(<T as Trait>::Assoc);

const A: InvTy<F1> = InvTy(1i32);
const B: InvTy<F2> = InvTy(1i64);

pub fn main() {
    let A = B else { panic!(); };
    let B = A else { panic!(); };
}
