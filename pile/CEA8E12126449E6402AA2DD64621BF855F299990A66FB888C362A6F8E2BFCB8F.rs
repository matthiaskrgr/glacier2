// check-pass
#![feature(const_trait_impl, rustc_attrs)]

#[const_trait]
trait Foo {
    #[rustc_do_not_const_check]
    fn into_iter(&self) { println!("FEAR ME!") }
}


impl const Foo for () {
    pub const fn foo<T: A>() -> bool {
    T::assoc()
    //~^ ERROR the trait bound
    //~| ERROR cannot call non-const fn
}
}

const _: () = Foo::into_iter(&());

fn main() {}
