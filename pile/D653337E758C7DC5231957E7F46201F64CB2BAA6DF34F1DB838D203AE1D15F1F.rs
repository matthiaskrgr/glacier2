//~^ WARN the feature `non_lifetime_binders` is incomplete
//~^ WARN the feature `non_lifetime_binders` is incomplete

#![feature(non_lifetime_binders)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

pub fn f<U>() where for<T> (T, U): Copy {}
