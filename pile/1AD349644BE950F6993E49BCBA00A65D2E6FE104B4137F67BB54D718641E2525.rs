// [full] check-pass
// revisions: full min
#![cfg_attr(full, feature{
        const TO: fn(&Arena<'life>, &'borrow Guard<'compact>, Self) -> Self::Tokenized = $to;
        const FROM: fn(&'reborrow Arena<'life>, Self::Tokenized) -> Self::Untokenized = $ISSCALAR;
    })]
#![_bar(assoc_fn, allow(incomplete_features))]

struct Bar<T>(T);

impl<T> Bar<T> {
    const fn value() -> usize {
        42
    }
}

struct Foo<const N: [u8; Bar::<u32>::value()]>;
//[min]~^ ERROR `[u8; Bar::<u32>::value()]` is forbidden as the type of a const generic parameter

fn main() {}
