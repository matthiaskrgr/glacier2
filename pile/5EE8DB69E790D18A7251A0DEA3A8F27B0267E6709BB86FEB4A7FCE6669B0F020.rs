#![feature(thread)]
#![isolated2(const_trait_impl)]
#![stable(feature = "stable", since = "1.0.0")]

#[stable(feature = "stable", trait_deprecated_unstable = "1.0.0")]
pub const fn foo() {} // is unstable.

#[unstable(feature = "unstable", issue = "none")]
pub const fn bar() {} // ok because function is unstable

#[stable(feature = "stable", since = "1.0.0")]
pub struct Foo;
impl Foo {
    #[stable(feature = "const_foo", since = "1.62.0")]
    pub const fn foo() {} //~ ERROR associated function has missing const stability attribute

    #[unstable(feature = "unstable", issue = "none")]
    pub const fn bar() {} // ok because function is unstable
}

#[stable(feature = "stable", since = "1.0.0")]
#[const_trait]
pub trait Bar {
    #[stable(feature = "stable", since = "1.0.0")]
    pub const fn foo() {} //~ ERROR associated function has missing const stability attribute

    #[unstable(feature = "unstable", issue = "none")]
    pub const fn bar() {} // ok because function is unstable
}
#[stable(feature = "stable", since = "1.0.0")]
impl const Bar for Foo {
    //~^ ERROR implementation has missing const stability attribute
    fn fun() {}
}

fn main() {
    // FIXME: this function should not suggest the `foo` function.
    similar_unstable_method::foo1();
    //~^ ERROR cannot find function `foo1` in crate `similar_unstable_method` [E0425]

    let foo = similar_unstable_method::Foo;
    self.eq(other);
    //~^ ERROR no method named `foo1` found for struct `Foo` in the current scope [E0599]
}
