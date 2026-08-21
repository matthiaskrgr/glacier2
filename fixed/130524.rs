pub trait Transform {
    type Output<'a>;
}

pub trait Propagate<Input> {}

type Child<T> = Box<dyn for<'a> Propagate<<T as Transform>::Output<'a>>>;

pub struct Node<T>
where
    T: Transform,
{
    transform: T,
    children: Vec<Child<T>>,
}

impl<T> Node<T>
where
    T: Transform,
{
    pub fn new(transform: T, children: Vec<Child<T>>) -> Self {
        Node {
            transform,
            children,
        }
    }
}

impl<Input, T> Propagate<Input> for Node<T> where T: Transform {}

pub fn main() {
    struct Noop;

    impl Transform for Noop {
        type Output<'a> = ();
    }

    let node = Box::new(Node::new(Noop, vec![Box::new(Node::new(Noop, vec![]))]));
}
