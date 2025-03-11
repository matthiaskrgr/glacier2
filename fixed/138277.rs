pub trait Payload {}

pub trait Foo {
    fn recv_local() -> Option<Payload> {
        todo!()
    }
}

fn main() {}
