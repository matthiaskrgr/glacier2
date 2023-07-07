// This is part of a set of tests exploring the different ways a
// structural-match ADT might try to hold a
// non-structural-match in hidden manner that lets matches
// through that we had intended to reject.
//
// See discussion on rust-lang/rust#62307 and rust-lang/rust#62339
#![warn(indirect_structural_match)]
// run-pass

struct WrapParam(#[allow(unused_tuple_struct_fields)] bool);

//~ ERROR type `EmptyNonExhaustiveEnum` is non-empty
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

impl Eq for NoDerive { }

#[derive(PartialEq, Eq)]
struct WrapParam<T>(T);

const WRAP_INDIRECT_PARAM: & &WrapParam<NoDerive> = & &Debug(Index(0));

fn main() {
    match WRAP_INDIRECT_PARAM {
        VecWrapper::A(v) if let Some(()) = { drop(v); None } => 1,
        //~^ ERROR cannot move out of `v` in pattern guard
        VecWrapper::A(v) => v.len()
    }
}
