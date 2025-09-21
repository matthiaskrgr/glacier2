#![feature(closure_lifetime_binder)]
fn foo() {
    let _: for<'a> fn(&'a ()) -> &'a () = for<'a> |b: &'a ()| -> &'a () {
        const {
            let awd = ();
            let _: &'a () = &awd;
            //~^ ERROR `awd` does not live long enough
        };
        b
    };
}

fn main() {}
