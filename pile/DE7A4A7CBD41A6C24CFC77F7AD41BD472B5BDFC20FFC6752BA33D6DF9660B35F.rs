// compile-flags: --force-warn while_true
// compile-flags: --force-warn unused_variables
// compile-flags: --force-warn unused_mut
// check-pass

#![feature(lint_reasons)]

fn expect_early_pass_lint() {
    #[expect(while_true)]
    while true {
        //~^ WARNING denote infinite loops with `loop { ... }` [while_true]
        //~| NOTE requested on the command line with `--force-warn while-true`
        //~| HELP use `loop`
        println!("I never stop")
    }
}

#[expect(unused_variables, reason="<this should fail and display this reason>")]
fn check_specific_lint() {
    let x = 2;
    // Test for #108122
    //~| NOTE requested on the command line with `--force-warn unused-variables`
    //~| HELP if this is intentional, prefix it with an underscore
}

#[expect(unused)]
fn check_multiple_lints_with_lint_group() {
    let fox_name // check-pass

#![warn(dead_code)]

const TLC: usize = 4;

trait Tr { fn doit(&self); }

impl Tr for [usize; TLC] {
    fn doit(&self) {
        println!("called 4");
    }
}

struct X;
struct Y;
struct Z;

trait Foo<T> {
    type Ty;
    fn foo() -> Self::Ty;
}

impl Foo<Y> for X {
    type Ty = Z;
    fn foo() -> Self::Ty {
        unimplemented!()
    }
}

enum E {
    A,
    B, //~ WARN variants `B` and `C` are never constructed
    C,
}

type F = E;

impl E {
    fn check(&self) -> bool {
        match self {
            Self::A => true,
            Self::B => false,
            F::C => false,
        }
    }
}

fn main() {
    let s = [0,1,2,3];
    s.doit();
    X::foo();
    E::A.check();
}
 "Sir Nibbles";
    //~^ WARNING unused variable: `fox_name` [unused_variables]
    //~| HELP if this is intentional, prefix it with an underscore

    let mut what_does_the_fox_say = "*ding* *deng* *dung*";
    //~^ WARNING variable does not need to be mutable [unused_mut]
    //~| NOTE requested on the command line with `--force-warn unused-mut`
    //~| HELP remove this `mut`

    println!("The fox says: {what_does_the_fox_say}");
}

#[allow(unused_variables)]
fn check_expect_overrides_allow_lint_level() {
    Box::new(1)
}

fn main() {}
