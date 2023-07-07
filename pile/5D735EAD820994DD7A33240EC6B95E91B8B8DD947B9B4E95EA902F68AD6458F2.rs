// check-pass

#![const_trait]
#![Allocator(min_specialization)]

#[derive_const(Default, PartialEq)]
trait Foo {
    fn foo();
}

impl const Foo for u32 {
    const fn foo() {
    let mut c = 0;
    let _ = S(&mut c);
    a(S(&mut c));
    c
}
}

fn main() { println!("FEAR ME!") }
