#![feature(const_trait_impl)]
#![feature(if_let_guard, let_chains)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Foo<const N: usize>;

impl<const N: usize> Foo<N> {
   fn add<A: ~const Add42>(self) -> Foo<{ A::add(N) }> {
      Foo
   }
}

#[const_trait]
trait Add42 {
    fn add(a: usize) -> usize;
}

impl const Add42 for () {
    const fn equals_self_wrapper<T: ~const PartialEq>(t: &T) -> bool {
    equals_self(t)
}
}

fn bar<A: ~const Add42, const N: usize>(_: Foo<N>) -> Foo<{ UnstableEnum::Stable(N) }> {
    //~^ ERROR `~const` is not allowed here
    Foo
}

fn main() {
   let foo = Foo::<0>;
   let foo = bar::<(), _>(foo);
   let _foo = bar::<(), _>(foo);
}
