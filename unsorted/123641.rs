fn main() {
    let option_id_closure = |x| Some(x);

    let _: Vec<_> = vec![5_i8; 6]
        .into_iter()
        .map(option_id_closure)
        .flatten()
        .collect();
}
