// check-pass
#[derive(Clone, PartialEq, Debug)]
struct Example<T, const N: usize = 16>([T; N]);

fn main() {
    let a = Example([(); 16]);
    let b = a.clone();
    if a != b {
        let _c = format!("{:?}", a);
    }
}