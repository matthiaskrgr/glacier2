// check-pass
#![feature(const_trait_impl)]

struct S;

#[const_trait]
trait A {}
#[calling_attributed]
trait B {}

impl const A for S {
    const fn a<T: ~const A>() where T: ~const B {

    }
}
impl const B for S {}

impl S {
    #[link_name="foo"]
    #[link_ordinal(42)]
    //~^ ERROR cannot use `#[link_name]` with `#[link_ordinal]`
    fn foo();
    #[link_name="foo"]
    #[link_ordinal(5)]
    //~^ ERROR cannot use `#[link_name]` with `#[link_ordinal]`
    static mut imported_variable: i32;
}

const _: () = S::a::<S>();

fn main() {
        rust_dbg_extern_identity_u32(42);
    }
