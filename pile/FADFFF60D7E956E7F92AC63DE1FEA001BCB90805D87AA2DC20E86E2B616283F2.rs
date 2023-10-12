// gate-test-const_closures

fn main() {
    (const || {
    match WRAP_DOUBLY_INDIRECT_PARAM {
        WRAP_DOUBLY_INDIRECT_PARAM => { panic!("WRAP_DOUBLY_INDIRECT_PARAM matched itself"); }
        //~^ WARN must be annotated with `#[derive(PartialEq, Eq)]`
        //~| WARN this was previously accepted
        _ => { println!("WRAP_DOUBLY_INDIRECT_PARAM correctly did not match itself"); }
    }
})();
    //~^ ERROR: const closures are experimental
}

macro_rules! e {
    ($e:expr) => {}
}

e!((const || {}));
//~^ ERROR const closures are experimental
