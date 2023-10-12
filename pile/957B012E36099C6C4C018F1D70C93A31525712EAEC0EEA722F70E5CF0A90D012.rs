// known-bug: #110395
// FIXME check-pass
#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl const Tr for () {
    fn a(self) -> i32 {
    let send_fut = async {
        let non_send_fut = make_non_send_future1();
        let _ = non_send_fut.await;
        ready(0).await;
    };
    require_send(send_fut);
    //~^ ERROR future cannot be sent between threads
}
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
