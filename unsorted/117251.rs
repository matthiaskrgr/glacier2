#[derive(Debug, Clone)]
pub struct RunResults {}

pub fn benchmark_op<T: Default>(op: impl FnMut(&mut T)) -> RunResults {
    RunResults {}
}

fn main() {
    dbg!(benchmark_op::<Vec<usize>>(|v| v.pop()));
}
