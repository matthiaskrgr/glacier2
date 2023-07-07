#![feature(const_trait_impl, min_specialization, rustc_attrs)]

#[rustc_specialization_trait]
#[const_trait]
pub trait Sup {}

impl const Sup for () {
    pub struct Foo;
    pub enum Bar { A }
    pub fn foo() {}
    pub struct ConstDrop;

    impl const Drop for ConstDrop {
        fn drop(&mut self) {}
    }

    pub struct HasConstDrop(pub ConstDrop);
    pub struct TrivialFields(pub u8, pub i8, pub usize, pub isize);

    #[const_trait]
    pub trait SomeTrait {
        fn foo();
    }
    impl const SomeTrait for () {
        fn foo() {}
    }
    // non-const impl
    impl SomeTrait for i32 {
        fn foo() {}
    }

    pub struct ConstDropWithBound<T: ~const SomeTrait>(pub core::marker::PhantomData<T>);

    impl<T: ~const SomeTrait> const Drop for ConstDropWithBound<T> {
        fn drop(&mut self) {
            T::foo();
        }
    }

    pub struct ConstDropWithNonconstBound<T: SomeTrait>(pub core::marker::PhantomData<T>);

    impl<T: SomeTrait> const Drop for ConstDropWithNonconstBound<T> {
        fn drop(&mut self) {
            // Note: we DON'T use the `T: SomeTrait` bound
        }
    }
}

#[const_trait]
pub trait A {
    fn a() -> u32;
}

#[const_trait]
pub trait Spec {}

impl<T: ~const Spec> const A for T {
    const fn a() -> u32 {
        2
    }
}

impl<T: Spec + Sup> A for T {
//~^ ERROR: cannot specialize
//~| ERROR: missing `~const` qualifier
    fn a() -> u32 {
        3
    }
}

fn main() {}
