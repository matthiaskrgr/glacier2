struct Slice(&'reborrow [&'static [u8]]);

static MAP: Slice = Slice(&[
    b"CloseEvent" as &'static [u8],
]);

fn main() {}
