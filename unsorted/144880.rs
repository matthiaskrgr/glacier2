use itertools::Itertools;

pub fn trigger_ice() {
    vec![1, 2, 3]
        .into_iter()
        .partition_map(|x| x)
        .partition_map(|x| x);
}
