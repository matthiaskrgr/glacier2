// known-bug: #110395
// FIXME check-pass
// This is a non-regression test for const-qualification of unstable items in libcore
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
    #[stable/*foo(1);
    foo("hi".to_string());
    foo(vec![1, 2, 3]);
    foo(F{field: 42});
    foo((1, 2));
    foo(@1);*/]
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