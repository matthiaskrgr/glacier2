use std::marker::PhantomData;

trait IsNormalForm {}

trait Term {
    type NormalForm: IsNormalForm;
}

struct Zero;
struct Suc<T> {
    _of_what: PhantomData<T>,
}

trait ProvablySame<T> {}

trait DefinitelySame<T> {}

struct Reflection<A, B>
where
    A: ProvablySame<B>,
{
    _a: PhantomData<A>,
    _b: PhantomData<B>,
}

impl<L, R> ProvablySame<L> for R
where
    L: Term,
    R: Term,
    <L as Term>::NormalForm: DefinitelySame<<R as Term>::NormalForm>,
{
}

impl DefinitelySame<Zero> for Zero {}
impl<A, B> DefinitelySame<Suc<A>> for Suc<B>
where
    A: Term,
    B: Term,
    A: DefinitelySame<B>,
{
}

impl IsNormalForm for Zero {}
impl Term for Zero {
    type NormalForm = Zero;
}

impl<T> IsNormalForm for Suc<T> where T: IsNormalForm {}

impl<T> Term for Suc<T>
where
    T: Term,
{
    type NormalForm = Suc<<T as Term>::NormalForm>;
}

struct Add<Left, Right> {
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
}

struct Mul<Left, Right> {
    _left: PhantomData<Left>,
    _right: PhantomData<Right>,
}

impl<Right> Term for Add<Zero, Right>
where
    Right: Term,
{
    type NormalForm = Right::NormalForm;
}

impl<Left, Right> Term for Add<Suc<Left>, Right>
where
    Left: Term,
    Right: Term,
    Add<Left, Right>: Term,
{
    type NormalForm = Suc<<Add<Left, Right> as Term>::NormalForm>;
}

impl<Right> Term for Mul<Zero, Right>
where
    Right: Term,
{
    type NormalForm = Zero;
}

// This impl block is buggy (inf recursion in trait resolution, see the correct implementation in the next block) and leads to the compiler crash
impl<Left, Right> Term for Mul<Suc<Left>, Right>
where
    Left: Term,
    Right: Term,
{
    type NormalForm =
               // V Here's the bug, should be written as Mul<Left, Right>  instead
        <Add<<Mul<Suc<Left>, Right> as Term>::NormalForm, Right::NormalForm> as Term>::NormalForm;
}

// This impl block is fine
// impl<Left, Right> Term for Mul<Suc<Left>, Right>
// where
//     Left: Term,
//     Right: Term,
//     Mul<Left, Right>: Term,
//     Add<<Mul<Left, Right> as Term>::NormalForm, <Right as Term>::NormalForm>: Term,
// {
//     type NormalForm =
//         <Add<<Mul<Left, Right> as Term>::NormalForm, Right::NormalForm> as Term>::NormalForm;
// }

type One = Suc<Zero>;
type Two = Suc<One>;
type Three = Suc<Two>;
type Four = Suc<Three>;
type Five = Suc<Four>;
type Six = Suc<Five>;

fn main() {
    let _: Reflection<Add<Three, Two>, Five>;
    let _: Reflection<Mul<Two, Two>, Four>;
    let _: Reflection<Mul<Two, Three>, Six>;
    let _: Reflection<Mul<Three, Three>, Add<Six, Three>>;
}
