// build-pass (FIXME(62277): could be check-pass?)

#![deny(unused)]

trait Trt {
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
pub struct Str {}
impl Trt for Str {}

macro_rules! check_impl {
    ($struct:ident,$trait:ident) => {
        const _ : () = {
            use std::marker::PhantomData;
            struct ImplementsTrait<T: $trait>(PhantomData<T>);
            let _ = ImplementsTrait::<$struct>(PhantomData);
            ()
        };
    }
}

const _ : () = ();

const _ : i32 = 42;
const _ : Str = Str{};

check_impl!(Str, Trt);
check_impl!(Str, Trt);

fn main() {
  check_impl!(Str, Trt);
  check_impl!(Str, Trt);
}
