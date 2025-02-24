fn hello() -> impl AsyncFnOnce<()> {
    async || {}
}

fn require_send(_: impl Send) {}

fn main() {
    require_send(hello()());
}
