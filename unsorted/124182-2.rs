struct LazyLock {
    data: ([u8], ()),
}

static EMPTY_SET: LazyLock = todo!();

fn main() {}
