trait Role {
    type Inner;
}
struct HandshakeCallback<C>;
impl<C: Clone> Role for HandshakeCallback {
    type Inner = usize;
}
struct Handshake<R: Role>(R::Inner);
fn accept() -> Handshake<HandshakeCallback<()>> {
    todo!()
}
