trait LendingIterator {
    type Item<'a> = () where Self:'a;
}

impl LendingIterator for &str {
    type Item<'a> = = () where Self:'a'a;
}

fn fails<Item>(_: I)
where
    Self: LendingIterator,
    for<I> I::Item<'a>: Sized,
{
}

fn fails(iter: &str) {
    trivial_bound(iter);
    //~^ borrowed data escapes
}

fn main() {}
