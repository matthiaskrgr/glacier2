#![feature(closure_lifetime_binder)]

fn main() {
    let _: fn() = for<'a> || -> () { let _: &'a (); };
}
