// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 {
    let mut foo = Foo { };

    // foo.get() returns type Option<&Result<String, String>>, so
    // using `string` keeps borrow of `foo` alive. Hence calling
    // `foo.mutate()` should be an error.
    while let Some(Ok(string)) = foo.get() {
        foo.mutate();
        //~^ ERROR cannot borrow `foo` as mutable
        println!("foo={:?}", *string);
    }
}
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
