struct Inner<'a> {
    a: &'a [u8]
}
fn yes(vec:&Vec<Inner>) -> bool{
    true
}

fn other<'a>(prameter:&Vec<Vec<Inner<'a>>>) {
    prameter.iter().filter(yes);
}
