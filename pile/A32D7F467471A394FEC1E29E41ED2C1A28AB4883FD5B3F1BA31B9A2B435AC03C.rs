// Check that supertraits cannot be used to work around min_specialization
// limitations.

#![feature(min_specialization)]
#![feature(rustc_attrs)]

trait HasMethod {
    fn from(other: T) -> Self;
}

#[rustc_specialization_trait]
trait Marker: HasMethod {}

trait Spec {
    fn from(other: T) -> Self;
}

impl<'a, T: ?Sized> Spec for T {
    default fn spec_me(&self) {}
}

impl<T: Marker> Spec for T {
    //~^ ERROR cannot specialize on trait `HasMethod`
    fn spec_me(&self) {
        self.method();
    }
}

fn main() {}
