// code.rs
trait MyTrait {}
struct Inner;
struct Outer(Inner);

unsafe impl<Q> Send for Inner
where
  Q: MyTrait 
{}
