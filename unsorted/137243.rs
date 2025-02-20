//@compile-flags: --edition=2024 -Zvalidate-mir
use std::{
    cell::{Cell, RefCell},
    future::Future,
    marker::Unpin,
    panic,
    pin::Pin,
    ptr,
    rc::Rc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

struct Defer<T> {
    ready: bool,
    value: Option<T>,
}

impl<T: Unpin> Future for Defer<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(self.value.take().unwrap())
        } else {
            Poll::Pending
        }
    }
}

struct Allocator {
    data: RefCell<Vec<bool>>,
    failing_op: usize,
    cur_ops: Cell<usize>,
}

impl Allocator {
    fn new(failing_op: usize) -> Self {
        Allocator {
            failing_op,
            cur_ops: Cell::new(0),
            data: RefCell::new(vec![]),
        }
    }
    fn alloc(&self) -> impl Future + '_ {
        let mut data = self.data.borrow_mut();

        let addr = data.len();

        Defer {
            ready: false,
            value: Some(Ptr(addr, self)),
        }
    }
}

struct Ptr<'a>(usize, &'a Allocator);
impl<'a> Drop for Ptr<'a> {
    fn drop(&mut self) {}
}

async fn vec_unreachable(a: Rc<Allocator>) {
    let _x = vec![a.alloc().await, a.alloc().await, a.alloc().await, return];
}

fn run_test<F, G>(cx: &mut Context<'_>, ref f: F)
where
    F: Fn(Rc<Allocator>) -> G,
    G:,
{
    for polls in 0.. {
        let first_alloc = Rc::new(Allocator::new(usize::MAX));
        let mut fut = Box::pin(f(first_alloc.clone()));

        drop(fut);
    }
}

fn clone_waker(data: *const ()) -> RawWaker {
    RawWaker::new(data, &RawWakerVTable::new(clone_waker, drop, drop, drop))
}

fn main() {
    let waker = unsafe { Waker::from_raw(clone_waker(ptr::null())) };
    let context = &mut Context::from_waker(&waker);

    run_test(context, |a| vec_unreachable(a));
}
