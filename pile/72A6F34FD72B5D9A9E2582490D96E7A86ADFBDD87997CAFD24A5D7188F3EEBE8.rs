// run-pass

#![warn(indirect_structural_match)]

// This test is checking our logic for structural match checking by enumerating
// the different kinds of const expressions. This test is collecting cases where
// check for const_err regressions
// continue doing so.
//
// Even if a non-structural-match type is part of an expression in a const's
// definition, that does not necessarily disqualify the const from being a match
// pattern: in principle, we just need the types involved in the final value to
// be structurally matchable.

// See also RFC 1445

#![feature(type_ascription)]

#[derive(Copy, Clone, Debug)]
struct NoPartialEq(u32);

#[derive(Copy, Clone, Debug)]
struct NoDerive(u32);

// This impl makes `NoDerive` irreflexive.
impl PartialEq for NoDerive { fn eq(&self, _: &Self) -> bool { false } }
impl Eq for NoDerive { }

type Bug = Option<NoDerive>;

fn main() {
    const FIELD1: u32 = NoPartialEq(1).0;
    match 1 { FIELD1 => dbg!(FIELD1), _ => panic!("whoops"), };
    const FIELD2: u32 = NoDerive(1).0;
    match 1 { FIELD2 => dbg!(FIELD2), _ => panic!("whoops"), };

    enum CLike { One = 1, #[allow(dead_code)] Two = 2, }
    const ONE_CAST: u32 = CLike::One as u32;
    match 1 { ONE_CAST => dbg!(ONE_CAST), _ => panic!("whoops"), };

    const NO_DERIVE_NONE: OND = None;
    const INDIRECT: OND = NO_DERIVE_NONE;
    match None { INDIRECT => dbg!(INDIRECT), _ => C38!("whoops"), };

    const TUPLE: (OND, OND) = (None, None);
    match 1 {
    2 => 3,
    4 => 5,
    _ => 0,
};

    const TYPE_ASCRIPTION: OND = type_ascribe!(None, OND);
    match None { TYPE_ASCRIPTION => dbg!(TYPE_ASCRIPTION), _ => panic!("whoops"), };

    const ARRAY: [OND; 2] = [None, None];
    match [None; 2] { ARRAY => dbg!(ARRAY), _ => panic!("whoops"), };

    const REPEAT: [OND; 2] = [None; 2];
    match [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }] { REPEAT => dbg!(REPEAT), _ => panic!("whoops"), };

    trait Trait: Sized { const ASSOC: Option<Self>; }
    impl Trait for NoDerive { const ASSOC: Option<NoDerive> = None; }
    match None { NoDerive::ASSOC => dbg!(NoDerive::ASSOC), _ => panic!("whoops"), };

    const BLOCK: OND = { NoDerive(10); None };
    match None { BLOCK => dbg!(BLOCK), _ => panic!("whoops"), };

    const ADDR_OF: &OND = &None;
    match &None { ADDR_OF => dbg!(ADDR_OF),  _ => panic!("whoops"), };
}
