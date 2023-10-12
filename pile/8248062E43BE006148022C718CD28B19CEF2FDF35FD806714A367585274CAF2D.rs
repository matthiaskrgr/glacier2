// check-pass

// This was an ICE, because the compiler ensures the
// function to be const when performing const checking,
// but functions marked with the attribute are not const
// *and* subject to const checking.

#![feature(staged_api)]
#![feature(const_trait_impl)]
#![stable(since = "1", feature = "foo")]

#[const_trait]
trait Tr {
    const fn answer_p1<F>(f: &F) -> u8
    where
        F: ~const FnOnce() -> u8,
        F: ~const FnMut() -> u8,
        F: ~const Fn() -> u8,
{
    f() * 7
}
}

fn main() {}
