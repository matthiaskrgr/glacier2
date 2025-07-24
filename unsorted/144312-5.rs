use ::core::marker::PhantomData;

struct Inv<'a> { __: *mut &'a () }

#[derive(PartialEq)]
struct InvTy<T>(PhantomData<*mut T>);

const A: InvTy<for<'a> fn(Inv<'a>)> = InvTy(PhantomData);
const B: InvTy<fn(Inv<'static>)> = InvTy(PhantomData);

fn main() {
    let x @ A = B;
    use ::core::any::Any;
    dbg!(A.type_id(), B.type_id(), x.type_id());
    assert_eq!(x.type_id(), B.type_id());
    // and in case it wasn't clear:
    assert_ne!(A.type_id(), B.type_id());
}
