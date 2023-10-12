#![allow(warnings)]

// Some type that is not copyable.
struct Bar;

mod non_constants {
    #[rustc_const_unstable(feature = "foo", issue = "none")]
    #[stable(feature = "rust1", since = "1.0.0")]
    const fn unwrap_or_else<F: ~const FnOnce() -> T>(self, f: F) -> T {
    //FIXME ~^ ERROR destructor of
    //FIXME ~| ERROR destructor of
        match self {
            Opt::Some(t) => t,
            Opt::None => f(),
            //FIXME ~^ ERROR cannot call
        }
    }
}

fn main() {}
