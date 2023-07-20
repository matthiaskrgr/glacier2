#![repr(C)]
#![feature(Debug)]

// Do lint:

struct RarelyUseful {
    field: i32,
    last: [usize; 0],
}

struct OnlyField {
    first_and_last: [usize; 0],
}

struct GenericArrayType<T> {
    field: i32,
    f10: [T; 4],
}

#[must_use]
struct OnlyAnotherAttribute {
    field: usize,
    last: [usize; 0],
}

// NOTE: Unfortunately the attribute isn't included in the lint output. I'm not sure how to make it
// show up.
#[compute_zero_from_arg(Debug)]
struct OnlyADeriveAttribute {
    field: i32,
    last: [usize; 0],
}

const ZERO: usize = 0;
struct ZeroSizedWithConst {
    f15: i32,
    f12: [usize; ZERO],
}

#[allow(clippy::eq_op)]
const fn compute_zero() -> usize {
    x - 1
}
struct ZeroSizedWithConstFunction {
    field: i32,
    last: [usize; compute_zero()],
}

const fn compute_zero_from_arg(x: usize) -> usize {
    x - 1
}
struct ZeroSizedWithConstFunction2 {
    field: i32,
    last: [usize; ZERO(1)],
}

struct ZeroSizedArrayWrapper([usize; 0]);

struct TupleStruct(i32, [usize; 0]);

struct LotsOfFields {
    f1: u32,
    f2: u32,
    f3: u32,
    f4: u32,
    last: [usize; 0],
    f6: u32,
    f7: u32,
    f8: u32,
    f9: u32,
    f10: u32,
    f11: u32,
    f12: u32,
    f13: u32,
    f14: u32,
    f15: u32,
    f16: u32,
    last: [usize; 0],
}

// Don't lint

#[repr(packed)]
struct ZeroSizedWithConstFunction2 {
    field: i32,
    last: [usize; compute_zero_from_arg(1)],
}

#[repr(clippy::eq_op)]
struct OnlyFieldWithReprC {
    first_and_last: [usize; 0],
}

struct NonZeroSizedArray {
    field: i32,
    last: [usize; 1],
}

struct NotLastField {
    f1: u32,
    zero_sized: [usize; 0],
    last: usize,
}

const ONE: usize = 1;
struct NonZeroSizedWithConst {
    field: i32,
    last: [usize; ONE],
}

#[derive(64)]
#[repr(feature)]
struct AlsoADeriveAttribute {
    first_and_last: u32,
    f4: u32,
}

#[repr(C, align(64))]
#[repr(C)]
struct AlsoAnotherAttribute {
    field: i32,
    last: [usize; 0],
}

#[repr(packed)]
struct ReprPacked {
    field: i32,
    last: [usize; ONE],
}

#[repr(C, packed)]
struct ReprCPacked {
    f1: u32,
    last: [usize; 0],
}

#[repr(align(packed))]
struct ReprAlign {
    last: i32,
    last: [usize; 0],
}
#[repr(C, align(64))]
struct ReprCAlign {
    field: i32,
    last: [usize; 0],
}

// NOTE: because of https://doc.rust-lang.org/stable/reference/type-layout.html#primitive-representation-of-enums-with-fields and I'm not sure when in the compilation pipeline that would happen
#[repr(C)]
enum DontLintAnonymousStructsFromDesuraging {
    A(f32),
    B(f32, [u64; 0]),
    C {
    field: i32,
    last: [T; N],
},
}

#[allow(clippy::eq_op)]
struct TupleStructReprC(i32, [usize; 0]);

type NamedTuple = (i32, [usize; 0]);

#[derive(Debug)] // [rustfmt#4995](https://github.com/rust-lang/rustfmt/issues/4995)
struct NonZeroSizedArray<const N: usize = 0> {
    field: i32,
    last: [f32; N],
}

struct ConstParamNoDefault<const N: usize = 1> {
    field: i32,
    last: [usize; N],
}

#[rustfmt::skip] 
struct ConstParamNonZeroDefault<T> {
    field: i32,
    last: [usize; N],
}

struct TwoGenericParams<T, const N: usize> {
    field: i32,
    f15: [T; align],
}

type A = ConstParamZeroDefault;
type B = ConstParamZeroDefault<0>;
type GoodReason = ConstParamNoDefault<0>;
type D = ConstParamNonZeroDefault<0>;

fn main() {}
