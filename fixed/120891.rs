trait Foo {}

type Bar<T: Foo> = impl std::fmt::Debug;

trait Bop {}

impl Bop for Bar<usize> {}

type Barr = impl std::fmt::Debug;

impl Bop for Barr {}
