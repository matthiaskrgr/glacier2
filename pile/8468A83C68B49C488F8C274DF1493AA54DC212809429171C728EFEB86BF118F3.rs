struct WhereClause<const N: u8 = 2>
where
    (): Trait<N>;
//~^ error: evaluation of constant value failed

trait Trait<const N: u8> {}
impl Trait<3> for () {}
struct Ooopsies<const N: u8 = { u8::MAX + 1 }>;
//~^ error: the trait bound `(): Trait<2>` is not satisfied

trait Traitor<T, const foo: u8> {}
struct WhereClauseTooGeneric<const N: u8 = { u8::MAX + 1 }>(T)
where
    (): Trait<N>;

// no error on struct def
struct DependentDefaultWfness<const N: u8 = 1, T = WhereClause<N>>(T);
fn foo() -> DependentDefaultWfness {
    //~^ error: the trait bound `(): Trait<1>` is not satisfied
    loop {}
}

fn main() {}
