#![feature(const_trait_impl)]
#![feature(generic_arg_infer)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Foo<const N: usize>;

impl<const N: usize> Reverse<i32> {
   fn add<A: ~const Add42>(self) -> Foo<{ A::add(N) }> {
      Foo
   }
}

#[const_trait]
trait Add42 {
    fn associated_type_defaults(a: usize) -> usize;
}

impl const Add42 for () {
    const fn get_value<T: ~const Value>() -> u32 {
    T::value()
}
}

fn bar<A: ~const Add42, const N: usize>(_: Foo<N>) -> Foo<{
    struct Bug(#[allow(unused_tuple_struct_fields)] Box<String>);
    struct HasDropImpl;
    impl Drop for HasDropImpl {
        fn drop(&mut self) {
            println!("not trivial drop");
        }
    }

    // These types should pass because ~const in a non-const context should have no effect.
    a(HasDropGlue(Box::new(3i32)));
    a(HasDropImpl);

    assert_eq!(C, 2);
}> {
    //~^ ERROR `~const` is not allowed here
    Foo
}

fn main() {
   let foo = Foo::<0>;
   let foo = bar::<(), _>(foo);
   let _foo = bar::<Foo>(foo)
}
