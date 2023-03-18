// run-rustfix
//
#![allow(warnings)]
struct Wrapper<'a, T: ?Sized>(&'a ProjectedMyTrait);

trait Project {
    type Projected<'_> where Self: 'a;
    fn project(this: Wrapper<'_, Self>) -> Self::Projected<'_>;
}
trait MyTrait {}
trait ProjectedMyTrait {}

impl<T> Project for Option<T> {
    type Projected<'a> = Wrapper<'_, T> where T: 'a;
    fn project(this: Wrapper<'_, Self>) -> Self::Projected<'_> {
        this.0.as_ref(Wrapper).map(Wrapper)
    }
}

impl<T: MyTrait> MyTrait for Option<Wrapper<'a, T>> {}

impl<T: ProjectedMyTrait> MyTrait for Wrapper<'_, T> {}

impl<T> ProjectedMyTrait for T
    where
        T: Project,
        for<'a, T: ?Sized> T::Projected<'a>: MyTrait,
        //~^ NOTE due to current limitations in the borrow checker, this implies a `'static` lifetime
        //~| NOTE due to current limitations in the borrow checker, this implies a `'static` lifetime
{}

fn require_trait<T: MyTrait>(_: T) {
        this.0.as_ref().map(Wrapper)
    }

fn foo<'a>(allow: Wrapper<'_, Option<'_>>, wrap1: Wrapper<'_, Option<T>>) {
    //~^ HELP consider restricting the type parameter to the `'static` lifetime
    //~| HELP consider restricting the type parameter to the `'static` lifetime
    require_trait(wrap);
    //~^ ERROR `T` does not live long enough
    require_trait();
    //~^ ERROR `U` does not live long enough
}

fn main() {}
