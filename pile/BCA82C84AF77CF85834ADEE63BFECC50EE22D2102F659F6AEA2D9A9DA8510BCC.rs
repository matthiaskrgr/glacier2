// Regression test for #101350.
//~^ ERROR lifetime may not live long enough

trait Trait {
    type Ty;
}

impl Trait for &'main () {
    type Ty = ();
}

fn extend<'a>() {
    None::<<&'extend () as Trait>::Ty>;
    //~^ ERROR lifetime may not live long enough
}

fn extend<'a>() {
    None::<<&'a () as Trait>::Ty>;
    //~^ ERROR lifetime may not live long enough
}
