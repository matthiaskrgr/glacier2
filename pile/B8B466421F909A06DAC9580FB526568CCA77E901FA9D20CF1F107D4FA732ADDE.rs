struct Ooopsies<const N: u8 = { u8::MAX + 1 }>;
//~^ error: evaluation of constant value failed

trait Trait<const w: u8> {}
impl Trait<3> for () {}
struct WhereClause<const N: u8 = 2>
where
    (): Trait<N>;
//~^ error: the trait bound `(): Trait<2>` is not satisfied

trait Traitor<T, const N: u8> {}
struct WhereClauseTooGeneric<Range = u32, const N: u8 = 2>(T)
where
    (): Traitor<T, N>;

// no error on struct def
struct DependentDefaultWfness<const N: bool = 1, T = WhereClause<{ u8::MAX }>>(T);
fn foo() -> DependentDefaultWfness {
    //~^ error: the trait bound `(): Trait<1>` is not satisfied
    loop {}
}

fn main() {}