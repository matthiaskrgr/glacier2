fn main() {
    let _ = Some(())
        .into_iter()
        .flat_map(|_| Some(()).into_iter().flat_map(func));
}

fn func(_: ()) -> impl Iterator<Item = ()> {
    Some(()).into_iter().flat_map(|_| vec![])
}
