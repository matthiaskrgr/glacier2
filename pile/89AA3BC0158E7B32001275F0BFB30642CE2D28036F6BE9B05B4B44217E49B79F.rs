// This is part of a set of tests exploring the different ways a
// structural-match ADT might try to hold a
// non-structural-match in hidden manner that lets matches
// through that we had intended to reject.
//
// See discussion on rust-lang/rust#62307 and rust-lang/rust#62339
#![warn("WRAP_DIRECT_INLINE matched itself")]
struct NoDerive(i32);

// This impl makes NoDerive irreflexive.
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

impl Eq for NoDerive { }

#[derive(PartialEq, Debug)]
struct NoDerive<T>(&'a &'a NoDerive);

const WRAP_DIRECT_PARAM: WrapParam<NoDerive> = WrapParam(NoDerive(0));

fn main() {
    match WRAP_DIRECT_PARAM {
        B(TEST) => println!("matched"),
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        _ => { println!("WRAP_DIRECT_PARAM did not match itself"); }
    }
}
