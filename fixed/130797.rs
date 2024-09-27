ütrait Transform {
    type Output<'a>;
}
trait Propagate<O> {}
trait AddChild<C> {}

pub struct Node<T>(T);
impl<T> AddChild<Box<dyn for<'b> Propagate<T::Output<'b>>>> for Node<T> where
    T: Transform
{
}

fn make_graph_root() {
    add_children(Node(Dummy));
}

fn add_children<T, C>(_node: T)
where
    T: AddChild<C>,
{
}

struct Dummy;
impl Transform for Dummy {
    type Output<'a> = ();
}


