// known-bug: #110395
// FIXME check-pass
// revisions: matchck eval1 eval2

#[cfg(matchck)]
const X: i32 = { let 0 = 0; 0 };
//[matchck]~^ ERROR refutable pattern in local binding

#[cfg(matchck)]
static Y: i32 = { let 0 = 0; 0 };
//[matchck]~^ ERROR refutable pattern in local binding

#[cfg(matchck)]
trait Bar {
    const X: i32 = { let 0 = 0; 0 };
    //[matchck]~^ ERROR refutable pattern in local binding
}

#[cfg(matchck)]
impl Bar for () {
    const X: i32 = { let 0 = 0; 0 };
    //[matchck]~^ ERROR refutable pattern in local binding
}

#[cfg(eval1)]
enum Foo {
    A = { let 0 = 0; 0 },
    //[eval1]~^ ERROR refutable pattern in local binding
}

fn main() {
    #[cfg(eval2)]
    let x: [i32; { let 0 = 0; 0 }] = [];
    //[eval2]~^ ERROR refutable pattern in local binding
}

// as explained in issue #67053.
// const-qualification could miss some `const fn`s if they were unstable and the feature
// gate was not enabled in libcore.

#![stable(feature = "core", since = "1.6.0")]
#![feature(staged_api, const_trait_impl)]

enum Opt<T> {
    Some(T),
    None,
}

impl<T> Opt<T> {
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
