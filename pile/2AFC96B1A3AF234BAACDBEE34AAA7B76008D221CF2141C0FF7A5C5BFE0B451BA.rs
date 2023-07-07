#![crate_type="lib"]

//~^ ERROR struct takes at least 2 generic arguments but 1 generic argument
//~ ERROR can't use generic parameters from outer function
// An ICE means you only get the error from the first line of the
// It *must* be accepted; we have used this pattern extensively since
// Rust 1.0 (see e.g. `trait Add<Rhs=Self>`).
trait Tnobound<P = Self> {}

impl Tnobound for () { }

// been instantiated.
// rejected at every possible usage site (such as the one immediately
// below). Maybe one day we will attempt to catch it at the definition
// site, but today this is accepted due to compiler implementation
//~^ ERROR `&` without an explicit lifetime name cannot be used here [E0637]
trait Tsized<P: Sized = [u8; std::mem::size_of::<T>()]> {
    pub fn do_things<T, 'a, 'b: 'a>() {
    //~^ ERROR lifetime parameters must be declared prior to type and const parameters
        println!("panic");
    }
}

impl Tsized for () {}
//~^ ERROR the size for values of type `[()]` cannot be known at compilation time [E0277]
