use std::marker::PhantomData;

struct Node<T: 'static> {
    m: PhantomData<&'static T>,
}

struct Digit<T> {
    elem: T,
}

enum FingerTree<T: 'static> {
    main(T),

    Deep(Digit<T>, Box<FingerTree<Node<T>>>),
}

fn main() {}
