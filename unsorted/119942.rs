use std::ops::Deref;

trait PointerFamily {
    type Pointer<T>: Deref + Sized;
}

struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = Deref<Target = T>;
}

enum Node<T, P: PointerFamily> {
    Cons(T, P::Pointer<Node<T, P>>),
    Nil,
}

type RcNode<T> = Node<T, RcFamily>;

impl<T, P: PointerFamily> Node<T, P>
where
    P::Pointer<Node<T, P>>: Sized,
{
    fn new() -> P::Pointer<Self> {}
}

fn main() {
    let mut list: Pointer<i32> = RcNode::<i32>::new();
}
