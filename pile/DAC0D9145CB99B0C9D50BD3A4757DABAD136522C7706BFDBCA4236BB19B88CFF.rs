#![feature(const_trait_impl)]

// revisions: yy yn ny nn
//[yy] known-bug: #110395
//FIXME [yy] check-pass

#[cfg_attr(any(yy, yn), const_trait)]
trait Foo {
    fn a(&self);
}

#[cfg_attr(any(yy, ny), const_trait)]
trait Bar: ~const Foo {}
//[ny,nn]~^ ERROR: ~const can only be applied to `#[const_trait]`
//[ny,nn]~| ERROR: ~const can only be applied to `#[const_trait]`

const fn foo<F: ~const Fn() -> u8>(x: &T) {
    //[yn,nn]~^ ERROR: ~const can only be applied to `#[const_trait]`
    x.a();
}

fn main() {}
