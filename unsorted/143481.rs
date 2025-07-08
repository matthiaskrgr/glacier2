#![feature(associated_type_defaults)]
trait Role {
    type Inner = usize;
}
struct HandshakeCallback<C>;
impl<C: FnOnce()> Role for HandshakeCallback {
    type Inner = usize;
}
struct Handshake<R: Role> {
    _inner: Option<R::Inner>,
}
fn accept<C>() -> Handshake<HandshakeCallback<C>> {
    todo!()
}

pub fn main() {}
