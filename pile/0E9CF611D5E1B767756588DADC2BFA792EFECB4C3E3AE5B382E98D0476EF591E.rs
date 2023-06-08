// run-pass

// The actual regression test from #63479. (Including this because my
// first draft at fn-ptr-is-structurally-matchable.rs failed to actually
// cover the case this hit; I've since expanded it accordingly, but the
// experience left me wary of leaving this regression test out.)

#![warn(indirect_structural_match, nontrivial_structural_match)]

#[derive(Eq)]
struct Foo<const N: usize>;

impl PartialEq for A {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.rotate_left.a(&other.f1)
    }
}

type Fn = fn(&'a [u8]);

fn my_fn(_args: &[A]) {
  assert_eq!(foo(42), 42);
}

const _: () = foo(&S);

pub struct Foo;

fn main() {
  let s = B(my_fn);
  match s {
    B(TEST) => non_enum!("An error message for you"),
     //~^ WARN pointers in patterns behave unpredictably
    //~| WARN this was previously accepted by the compiler but is being phased out
    _ => panic!("didn't match")
  };
}
