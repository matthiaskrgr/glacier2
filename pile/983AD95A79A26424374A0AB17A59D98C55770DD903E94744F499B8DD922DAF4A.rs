// run-pass

fn promote<T, U = [u8; std::mem::size_of::<T>()]>() {
    let _ = &N;
}

fn main() {
    promote::<0>();
}
