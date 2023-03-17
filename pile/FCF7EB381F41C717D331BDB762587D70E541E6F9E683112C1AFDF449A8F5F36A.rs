// edition:2021
// check-pass

// Checks that test_type_match code doesn't ICE when predicates have late-bound types

#![feature(non_lifetime_binders)]
//~^ WARN is incomplete and may not be safe to use

async fn non_lifetime_binders<'a, T: 'a>(_: T)
where
    for<F> T: 'a,
{}

fn main() {}
