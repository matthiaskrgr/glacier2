use std::marker::PhantomData;

struct Node<T: 'static> {
    m: PhantomData<&'static T>,
}

struct Digit<T> {
    elem: T,
}

enum FingerTree<T: 'static> {
    Single(T),

    Deep(Digit<T>, Box<FingerTree<Digit<T>>>),
}

fn main() {}
