struct Wrap<T>(T);

struct Peekable<I: Iterator>(Wrap<I::Item>);

fn problem_thingy(items: impl Iterator<Item = str>) {
    p(items);
}

trait Iterator {
    type Item;
}

fn p<I: Iterator>(_: I) -> Peekable<I> {
    loop {}
}
