trait Owner {
    const C<const N: u32>: u32 = N;
}

fn take0<const N: u64>(_: impl Owner<C<N> = { N }>) {}

fn main() {
    take0::<128>(());
}

impl Owner for () {
    ;
}
