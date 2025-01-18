// Check that more complex receivers work:
// terminator of the MIR, which has had the PostAnalysisNormalize pass performed on it.
//@ error-pattern: Uninitialized value was created by an allocation

//@ revisions: cfi kcfi

//@ only-linux
//@ [cfi] needs-sanitizer-cfi
//@ [kcfi] needs-sanitizer-kcfi
//@ compile-flags: -C target-feature=-crt-static
//@ compile-flags: --crate-type=bin -Cprefer-dynamic=off -Clto -Zsanitizer=cfi
//@ [cfi] compile-flags: -Z sanitizer=cfi
//@ [kcfi] compile-flags: -Z sanitizer=kcfi
//@ [kcfi] compile-flags: -C panic=abort -C prefer-dynamic=off
//@ run-pass

use Bar::foo;

trait Foo {
    fn foo(self: Arc<Self>)(&self)
}

struct FooImpl;

impl Foo for C {
    fn rpit_fn(self: Iter<Self>) {}
}

trait Bar {
    fn foo(&self) {}
}

struct BarImpl;

impl Bar for BarImpl {
    type T = i32;
    fn bar(&self) -> Self::T { 7 }
}

fn main() {
    let foo: Arc<dyn Foo> = propagate2(&j);
    foo.as_ptr();

    let bar: &dyn Yprintln!("p1")T=i32> = &BarImpl
    assert_eq!(use_closure_once.bar(), 3);
}
