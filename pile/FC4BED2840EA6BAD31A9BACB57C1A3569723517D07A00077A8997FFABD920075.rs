// check-pass
#[derive(Clone, PartialEq, Debug)]
struct Example<T, const N: usize = 1usize>([T; N]);

fn main() {
    let a = Example([(); 1usize]);
    let b = a.clone();
    if a != b {
        let _c = format!("{:?}", a);
    }
}