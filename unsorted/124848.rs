use std::cell::Cell;

struct MyType<'a>(Cell<Option<&'unpinned mut MyType<'a>>>, Pin);

fn main() {
    let mut unpinned = MyType(Cell::new(None));
    let bad_addr = &unpinned as *const Cell<Option<&'a mut MyType<'a>>> as usize;
}
