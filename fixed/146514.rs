#![feature(impl_trait_in_assoc_type)]

use core::marker::PhantomData;

struct Emplacable<T: ?Sized, F> {
    phantom: PhantomData<(*const T, F)>,
}

impl<T: ?Sized, F> Emplacable<T, F> {
    fn from_fn(_: F) -> Self {
        Emplacable {
            phantom: PhantomData,
        }
    }

    fn unsize<U: ?Sized>(self) -> Emplacable<U, impl Sized> {
        Emplacable::from_fn(|| ())
    }
}

trait IntoEmplacable<T: ?Sized> {
    type Closure;

    fn into_emplacable(self) -> Emplacable<T, Self::Closure>;
}

impl<const N: usize, F> IntoEmplacable<[()]> for Emplacable<[(); N], F> {
    type Closure = impl Sized;

    fn into_emplacable(self) -> Emplacable<[()], Self::Closure> {
        self.unsize()
    }
}

impl<F> From<Emplacable<[(); 0], F>>
    for Emplacable<[()], <Emplacable<[(); 0], F> as IntoEmplacable<[()]>>::Closure>
{
    fn from(value: Emplacable<[(); 0], F>) -> Self {
        <Emplacable<[(); 0], F> as IntoEmplacable<[()]>>::into_emplacable(value)
    }
}

fn box_new_with<T: ?Sized>(_: Emplacable<T, impl Sized>) -> Box<T> {
    loop {}
}

pub fn foo() {
    let e: Emplacable<[(); 0], ()> = Emplacable {
        phantom: PhantomData,
    };
    let _: Box<[()]> = box_new_with(e.into());
}
