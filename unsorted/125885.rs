#![feature(type_alias_impl_trait,transmutability)]
trait OpaqueTrait {}
trait AnotherTrait {}

type OpaqueType = impl OpaqueTrait;
impl AnotherTrait for OpaqueType {}

impl<T: std::mem::BikeshedIntrinsicFrom<()>> AnotherTrait for T {}

pub fn main() {}
