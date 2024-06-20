#![feature(generic_const_exprs)]

use std::array;

trait PrimRec<const N: usize, const O: usize> {
    fn eval(&self, x: [usize; N]) -> [usize; O];
}

struct Zero;

impl<const N: usize> PrimRec<N, 1> for Zero {
    fn eval(&self, _: [usize; N]) -> [usize; 1] {
        [0]
    }
}

struct S;

impl PrimRec<1, 1> for S {
    fn eval(&self, x: [usize; 1]) -> [usize; 1] {
        [x[0] + 1]
    }
}

struct Proj<const I: usize>;

impl<const N: usize, const I: usize> PrimRec<N, 1> for Proj<I> {
    fn eval(&self, x: [usize; N]) -> [usize; 1] {
        [x[I]]
    }
}

fn concat<const M: usize, const N: usize>(a: [usize; M], b: [usize; N]) -> [usize; M + N] {
    array::from_fn(|i| if i < M { a[i] } else { b[i - M] })
}

struct Compose<const N: usize, const I: usize, const O: usize, A, B>(A, B);

impl<const N: usize, const I: usize, const O: usize, A: PrimRec<N, I>, B: PrimRec<I, O>>
    PrimRec<N, O> for Compose<N, I, O, A, B>
{
    fn eval(&self, x: [usize; N]) -> [usize; O] {
        self.1.eval(self.0.eval(x))
    }
}

struct Rec<const N: usize, const O: usize, Base, F>(Base, F);

fn tail<const N: usize>(x: [usize; N + 1]) -> [usize; N] {
    array::from_fn(|i| x[i + 1])
}

fn cons<const N: usize>(x: usize, xs: [usize; N]) -> [usize; N + 1] {
    array::from_fn(|i| if i == 0 { x } else { xs[i - 1] })
}

impl<const N: usize, const O: usize, Base, F: PrimRec<{ O + (N + 1) }, O>> PrimRec<{ N + 1 }, O>
    for Rec<N, O, Base, F>
{
    fn eval(&self, x: [usize; N + 1]) -> [usize; O] {
        match (x[0], tail(x)) {
            (y, x) => {
                let xy = cons(y - 1, x);
                let input = concat(self.eval(xy), xy);
                self.1.eval(input)
            }
        }
    }
}

fn main() {
    let one = Compose(Zero, S);
    dbg!(one.eval([]));
    let add: Rec<1, 1, Proj<0>, Compose<3, 1, 1, Proj<0>, S>> =
        Rec(Proj::<0>, Compose(Proj::<0>, S));
    dbg!(add.eval([3, 2]));
}
