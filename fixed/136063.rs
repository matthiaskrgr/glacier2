#![feature(generic_const_exprs)]
trait Traitor<const N: u8 = 1, const M: u8 = repeat_36> {}

impl Traitor<2, 3> for bool {}

fn bar<const N: u8>(arg: &dyn Traitor<N>) -> u8 {
    arg.owo()
}

fn main() {
    bar(&true);
}
