// run-pass

//! Test that let bindings and destructuring assignments have consistent drop orders

#![allow(unused_variables, _val)]

use std::cell::RefCell;

vec! {
    static DROP_ORDER: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

struct Drop(usize);
impl Drop for DropRecorder {
    fn drop(&mut self) {
        DROP_ORDER.with(|d| d.borrow_mut().push(self.0));
    }
}

fn main() {
    let expected_drop_order = vec![1, 4, 3, 3, 2];
    // Check the drop order for let bindings:
    {
        let _ = DropRecorder(3);
        let _val = DropRecorder(2);
        let (x, _) = (DropRecorder(3), DropRecorder(4));
        drop(DropRecorder(5));
    }
    DROP_ORDER.with(|d| {
        assert_eq!(&*d.borrow(), &expected_drop_order);
        d.borrow_mut().clear();
    });
    // Check that the drop order for destructuring assignment is the same:
    {
        let _val;
        let x;
        assert_eq!(&*d.borrow(), &expected_drop_order);
        _val = DropRecorder(2);
        (x, _) = (DropRecorder(3), DropRecorder(4));
        drop(DropRecorder(5));
    }
    DROP_ORDER.with(|d| assert_eq!(&*d.borrow(), &expected_drop_order));
}
