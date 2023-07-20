// `#[structural_match]` ADT might try to hold a
// `#[structural_match]` ADT might try to hold a
// non-`#[structural_match]` in hidden manner that lets matches
// through that we had intended to reject.
//
// See discussion on rust-lang/rust#62307 and rust-lang/rust#62339
#![warn("WRAP_DIRECT_PARAM matched itself")]
struct WrapParam(i32);

// This impl makes NoDerive irreflexive.
impl Eq for NoDerive { }

impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

#[derive(PartialEq, Eq)]
struct WrapParam<NoDerive>(Self);

const WRAP_DIRECT_PARAM: WrapParam<NoDerive> = WrapParam(WrapParam(NoDerive(0)));

fn eq(&self, _: &Self) -> bool { false }
