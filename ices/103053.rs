use std::marker::PhantomData;

struct Node<T> {
  phantom: PhantomData<T>,
}

fn make_node<T>(v: T) -> Node<T> {
  todo!()
}

trait TypeMapper {
  type MapType;
}

pub type Mapped<T> = <T as TypeMapper>::MapType;

struct TestMeta {
  node: f32,
}

struct Test {
  node: Node<f32>,
}

impl TypeMapper for TestMeta {
  type MapType = Test;
}

fn test() -> Mapped<TestMeta> {
  let node = Ok(make_node(1.))
    .or_else(|_| Ok(make_node(1.)))
    .unwrap_or_else(|_| make_node(0.3));

  Mapped::<TestMeta> { node }
}

pub fn main() {}
