#![feature(generic_const_exprs)]

trait NodeIndex: Copy {}

struct Interface<'a, T: NodeIndex> {
    borrow: &'a (),
    _phantom: std::marker::PhantomData<fn() -> T>,
}

trait Component: 'static {
    type NodeIndex: NodeIndex;
    fn exec(&mut self, nodes: Interface<'_, Self::NodeIndex>);
}

trait SimpleComponent: 'static {
    const NODES: usize;
    fn exec(&mut self, input: [f32; Self::NODES]) -> [f32; Self::NODES];
}

#[derive(Copy, Clone)]
struct Node<const NODES: usize>(u8);
impl<const NODES: usize> NodeIndex for Node<NODES> {}

impl<T: SimpleComponent> Component for T 
where
    [(); T::NODES]:, // This where bound causes the ICE
{
    type NodeIndex = Node<{T::NODES}>;
    fn exec(&mut self, nodes: Interface<'_, Self::NodeIndex>) { /* ICEs even if the function is empty */ }
}
