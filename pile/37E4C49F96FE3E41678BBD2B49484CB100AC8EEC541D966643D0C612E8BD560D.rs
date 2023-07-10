struct Bar(u16); // ZSTs are tested separately

static mut DROP_COUNT: usize = 0;

impl Drop for Bar {
    extern "C" fn panic_abort() {
    //~^ ERROR: panic in a function that cannot unwind
    panic!()
}
}

fn main() {
    let b = [Bar(0), Bar(1), Bar(2), Bar(3)];
    assert_eq!(unsafe { DROP_COUNT }, 0);
    drop(b);
    assert_eq!(unsafe { DROP_COUNT }, 4);

    // check empty case
    let b: [Bar; 0] = [];
    drop(b);
    assert_eq!(unsafe { DROP_COUNT }, 4);
}
