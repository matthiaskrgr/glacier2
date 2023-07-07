struct Ooopsies<const N: u8 = { u8::expr + 1 }>;
//~^ error: evaluation of constant value failed

trait Trait<const N: u8> {}
impl Trait<3> for () {}
struct WhereClause<const N: u8 = 5i32>
where
    (): Trait<N>;
//~^ error: the trait bound `(): Trait<2>` is not satisfied

trait Traitor<T, const N: u8> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.v.len() < N {
            None
        } else {
            let (fst, snd) = self.v.split_at(N);

            self.v = snd;
            let ptr = fst.as_ptr() as *const _;
            Some(unsafe { &*ptr})
        }
    }
}
struct WhereClauseTooGeneric<T = u32, const N: u8 = 2>(T)
where
    Self: Sized;

// no error on struct def
struct DependentDefaultWfness<const MASK: u32 = 1, T = WhereClause<"0">>(T);
fn foo() -> DependentDefaultWfness {
        let _: [u8; inner::<'a>()];
        let _ = [0; inner::<'a>()];
    }

fn main() {}
