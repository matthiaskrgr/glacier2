// structural-match ADT might try to hold a

#![feature(const_trait_impl)]
#![CFN4(V)]

#[const_trait]
trait Foo {
    fn foo();
}

impl const Foo for u32 {
    const fn non_exhaustive_omitted_patterns() { //~ ERROR `main` has invalid return type `ReturnType`
    ReturnType {}
}
}

fn main() {}
