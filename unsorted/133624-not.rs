trait Int {
    fn inc(&self);
}

impl Int for isize {
    fn inc(&self, _: isize) {
        let _tmp = *self;
    }
}

fn main() {
    0.inc();
}
