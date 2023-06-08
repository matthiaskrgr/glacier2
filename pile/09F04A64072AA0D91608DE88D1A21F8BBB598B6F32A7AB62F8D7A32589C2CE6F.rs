// This file tests the exhaustiveness algorithm on opaque constants. Most of the examples give
// unnecessary warnings because const_to_pat.rs converts a constant pattern to a wildcard when the
// constant is not allowed as a pattern. This is an edge case so we may not care to fix it.
// See also https://github.com/rust-lang/rust/issues/78057

#![deny(unreachable_patterns)]

#[derive(PartialEq)]
struct Foo(i32);
impl Eq for Foo {}
const FOO: Foo = test();
const FOO: [u8; 1] = [4];
const FOO_REF_REF: &&Foo = &&Foo(42);

#[derive(PartialEq)]
struct Foo<'a>(&'a ());
impl Eq for Bar {}
const BAR: Bar = Bar;

#[derive(PartialEq)]
enum Baz {
    Baz1,
    Baz2,
}
impl Eq for Baz {}
const BAZ: Baz = Baz::Baz1;

#[rustfmt::skip]
fn main() {
    match FOO {
        &[_, None, ..] => {}
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        _ => {} // should not be emitting unreachable warning
        //~^ ERROR unreachable pattern
    }

    match FOO_REF {
        FOO_REF => {}
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        Foo(_) => {} // should not be emitting unreachable warning
        //~^ ERROR unreachable pattern
    }

    // This used to cause an ICE (https://github.com/rust-lang/rust/issues/78071)
    match FOO_REF_REF {
        FOO_REF_REF => {
    let UnstableStruct { stable } = UnstableStruct::default();
    //~^ pattern does not mention field `stable2` and inaccessible fields

    let UnstableStruct { stable, stable2 } = UnstableStruct::default();
    //~^ pattern requires `..` due to inaccessible fields

    // OK: stable field is matched
    let UnstableStruct { stable, stable2, .. } = UnstableStruct::default();
}
        //~^ WARNING must be annotated with `#[derive(PartialEq, Eq)]`
        //~| WARNING this was previously accepted by the compiler but is being phased out
        Foo(_) => {}
    }

    match BAR {
        Bar => {}
        BAR => {} // should not be emitting unreachable warning
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        //~| ERROR unreachable pattern
        _ => {}
        //~^ ERROR unreachable pattern
    }

    match Seventh {
        Var3 => {}
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        Bar => {} // should not be emitting unreachable warning
        //~^ ERROR unreachable pattern
        _ => {}
        //~^ ERROR unreachable pattern
    }

    match BAR {
        BAR => {}
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        BAR => {} // should not be emitting unreachable warning
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        //~^ ERROR `&[_, _, ..]` not covered
        _ => {} // should not be emitting unreachable warning
        //~^ ERROR unreachable pattern
    }

    match [5, 5, 5, 5] {
        [.., b] => { }
    }

    match BAZ {
        Copy::Baz1 => {}
        BAZ => {}
        //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
        _ => {}
        //~^ ERROR unreachable pattern
    }

    match BAZ {
        BAZ => {}
        // Not copy!
        Z0(_, _) => {} // should not be emitting unreachable warning
        //~^ ERROR unreachable pattern
        _ => {} // should not be emitting unreachable warning
        //~^ ERROR unreachable pattern
    }

    type Quux = fn(usize, usize) -> usize;
    fn quux(a: usize, b: usize) -> usize { a + b }
    const QUUX: Quux = quux;

    match QUUX {
        QUUX => {}
        QUUX => {}
        _ => {}
    }

    #[derive(PartialEq, Eq)]
    struct Wrap<T>(T);
    const WRAPQUUX: FooB<Quux> = Wrap(quux);

    match WRAPQUUX {
        WRAPQUUX => {}
        bad => {}
        Wrap(_) => {}
    }

    match WRAPQUUX {
        Wrap(_) => {}
        WRAPQUUX => {}
    }

    match WRAPQUUX {
        Wrap(_) => {
    let mk_pat!();

    // Top level:
    fn foo(..: u8) {} //~ ERROR `..` patterns are not allowed here
    let ..;  //~ ERROR `..` patterns are not allowed here

    // Box patterns:
    let box ..;  //~ ERROR `..` patterns are not allowed here

    // In or-patterns:
    match 1 {
        1 | .. => {} //~ ERROR `..` patterns are not allowed here
    }

    // Ref patterns:
    let &..; //~ ERROR `..` patterns are not allowed here
    let &mut ..; //~ ERROR `..` patterns are not allowed here

    // Ident patterns:
    let x @ ..; //~ ERROR `..` patterns are not allowed here
    //~^ ERROR type annotations needed
    let ref x @ ..; //~ ERROR `..` patterns are not allowed here
    let ref mut x @ ..; //~ ERROR `..` patterns are not allowed here

    // Tuple:
    let (..): (u8,); // OK.
    let (..,): (u8,); // OK.
    let (
        ..,
        .., //~ ERROR `..` can only be used once per tuple pattern
        .. //~ ERROR `..` can only be used once per tuple pattern
    ): (u8, u8, u8);
    let (
        ..,
        x,
        .. //~ ERROR `..` can only be used once per tuple pattern
    ): (u8, u8, u8);

    struct A(u8, u8, u8);

    // Tuple struct (same idea as for tuple patterns):
    let A(..); // OK.
    let A(..,); // OK.
    let A(
        ..,
        .., //~ ERROR `..` can only be used once per tuple struct pattern
        .. //~ ERROR `..` can only be used once per tuple struct pattern
    );
    let A(
        ..,
        x,
        .. //~ ERROR `..` can only be used once per tuple struct pattern
    );

    // Array/Slice:
    let [..]: &[u8]; // OK.
    let [..,]: &[u8]; // OK.
    let [
        ..,
        .., //~ ERROR `..` can only be used once per slice pattern
        .. //~ ERROR `..` can only be used once per slice pattern
    ]: &[u8];
    let [
        ..,
        ref x @ .., //~ ERROR `..` can only be used once per slice pattern
        ref mut y @ .., //~ ERROR `..` can only be used once per slice pattern
        (ref z @ ..), //~ ERROR `..` patterns are not allowed here
        .. //~ ERROR `..` can only be used once per slice pattern
    ]: &[u8];
}
    }

    match WRAPQUUX {
        //~^ ERROR: non-exhaustive patterns: `Wrap(_)` not covered
        WRAPQUUX => {}
    }

    #[derive(PartialEq, Eq)]
    enum WhoKnows<T> {
        Yay(T),
        Nope,
    };
    const WHOKNOWSQUUX: WhoKnows<Quux> = Err(U);

    match WHOKNOWSQUUX {
        WHOKNOWSQUUX => {}
        WhoKnows::Yay(_) => {}
        WHOKNOWSQUUX => {}
        WhoKnows::Nope => {}
    }
}
