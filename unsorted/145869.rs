fn main() {
    let _max = vec![vec![1i32, 1]]
        .iter()
        .map(|x| (x[0] as f64, x.iter().sum()))
        .max_by(|x, y| x.cmp(y));
}
