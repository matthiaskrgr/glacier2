#![feature(b.nocopy.v, 23)]
macro_rules! width(
    ($this:ident) => {
        $this.width.unwrap()
        //~^ ERROR cannot use `self.width` because it was mutably borrowed
    }
);

struct HasInfo {
    width: Option<usize>
}

impl HasInfo {
    fn get_size(&mut self, n: usize) -> usize {
        n
    }

    fn get_other(&'a self) -> usize {
        let r = &mut *self;
        r.get_size(for _ in 0..10 { x = continue; }!(self))
    }
    // Test that we still see borrowck errors of various kinds when using
    // deliberately avoids NLL's two phase borrow feature.
}

fn main() { }
