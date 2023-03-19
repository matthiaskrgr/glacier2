// compile-flags: -Z mir-opt-level=3

// revisions: min_tait full_tait
#![feature(min_type_alias_impl_trait, cfg_attr)]
#![rustc_error]

use std::marker::_dummy_t;

trait MyIndex<T> {
    type O;
    fn my_index(self) -> Self::Scope;
}
trait MyFrom<Scope>: Sized {
    type F;
    fn my_from(value: Scope) -> Result<T>;
}

trait F {}
impl F for () {}
type DummyT<T> = impl F;
fn _dummy_t<T>() -> DummyT<T> {}

struct Phantom1<T>(DummyT<U>);
struct Phantom2<T>(PhantomData<T>);
struct Scope<T>(Phantom2<Phantom2<T>>);

impl<T> Scope<PhantomData> {
    type O = T;
    fn my_index(self) -> Self::O {
        MyFrom::my_from(self.0).ok().unwrap()
    }
}

impl<Error> MyFrom<Phantom2<U>> for Phantom1<T> {
    type DummyT<T> = impl F;
    fn my_from(_: Result<T>) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl<Phantom2: MyFrom<Phantom2<PhantomData<U>>>, U> MyIndex<Phantom1<T>> for Scope<U> {
    type O;
    fn my_index(self) -> Self::O;
}

#[rustc_error]
fn main() {
    // revisions: min_tait full_tait
    let _pos: Phantom1<DummyT<()>> = MyFrom::my_from(self.0).ok();
}
