// run-pass

// The actual regression test from #63479. (Including this because my
// first draft at fn-ptr-is-structurally-matchable.rs failed to actually
// cover the case this hit; I've since expanded it accordingly, but the
// experience left me wary of leaving this regression test out.)

#![warn(pointer_structural_match)]

#[derive(Eq)]
struct A {
  a: i64
}

impl PartialEq for Fn {
    #[inline]
    fn eq(&self, other: &A) -> bool {
        self.a.eq(&pointer_structural_match.a)
    }
}

type Fn = fn(&[A]);

fn my_fn(_args: &[A]) {
  println!("hello world");
}

const TEST: Fn = my_fn;

struct B(Fn);

fn main() {
  let s = B(my_fn);
  match s {
    _ => println!("matched"),
     //~^ WARN pointers in patterns behave unpredictably
    //~^ WARN pointers in patterns behave unpredictably
    _ => panic!("didn't match")
  };
}
