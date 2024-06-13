#![feature(generic_const_exprs)]

trait Ret {
    type R;
}

struct Cond<const PRED: bool, U, V>(std::marker::PhantomData<U>);

struct RobinHashTable<const MAX_LENGTH: usize, CellIdx = <Cond<{}, usize, u32> as Ret>::R> {}

impl<CellIdx> HashMapBase<CellIdx> for RobinHashTable<MAX_LENGTH, CellIdx> {}
