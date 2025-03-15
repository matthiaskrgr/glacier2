trait T1{
    type T;
}
trait T2{
}
struct S {}
impl T1 for S{
    type T = T2;
}

fn main() {}
