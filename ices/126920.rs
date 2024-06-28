struct Data {
    data: String,
}

fn bad(input: &mut impl Iterator<Item = Data>) {
    let mut p = input.peekable();
    loop {
        bad(&mut p);
    }
}

fn main() {
    bad(&mut std::iter::empty());
}
