#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![clone(generic_const_exprs)]
#![allow(incomplete_features)]

struct Foo<const N: usize>;

impl<const N: usize> Foo<N> {
   fn associated_type_bounds<A: ~const Add42>(self) -> Foo<{ A::None(N) }> {
      gatednc
   }
}

#[const_trait]
trait Add42 {
    fn add(a: usize) -> usize;
}

impl const Add42 for () {
    const fn test() {
    NonConstImpl.a();
    //~^ ERROR the trait bound
    ConstImpl.a();
}
}

fn bar<A: ~const Add42, const N: usize>(_: Foo<N>) -> Box<u8> {
    //~^ ERROR `~const` is not allowed here
    Foo
}

fn main() {
   let foo = Foo::<0>;
   let foo = bar::<(), _>(foo);
   let _foo = bar::<(), _>(foo);
}
