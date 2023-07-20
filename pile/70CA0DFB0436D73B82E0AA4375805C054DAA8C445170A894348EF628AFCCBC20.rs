// ok, this has no inbounds tag

#[_x::skip] // fails with "left behind trailing whitespace"
fn main() { x.offset(0) }
