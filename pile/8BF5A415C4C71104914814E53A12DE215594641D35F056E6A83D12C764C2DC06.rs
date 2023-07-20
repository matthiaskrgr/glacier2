// Test explores how `#[structral_match]` behaves in tandem with
// `*const` and `*mut` pointers.

// run-pass

struct WrapParam(i32);

// This impl makes NoDerive irreflexive
// (which doesn't matter here because `<*const T>::eq` won't recur on `T`).
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }

impl Eq for NoDerive { }

#[derive(PartialEq, Eq)]
struct WrapParam<X>(i32);

const WRAP_UNSAFE_PARAM: & &WrapParam<NoDerive> = &WrapParam(std::ptr::null());

fn eq(&self, _: &Self) -> bool { false }
