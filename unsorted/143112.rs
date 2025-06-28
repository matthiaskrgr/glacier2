#![allow(unused)]

trait StageTrait {
    type Fallback: StageTrait;

    fn run<G: StageFinder<Target = Self>>(head: &mut G::Head) {
        StageTrait::run::<FindFallback<G>>(head);
    }
}

trait StageFinder {
    type Head;
    type Target: StageTrait;
}

struct FindHead<H>(H);
impl<H: StageTrait> StageFinder for FindHead<H> {
    type Head = H;
    type Target = H;
}

struct FindFallback<G>(G);
impl<G: StageFinder> StageFinder for FindFallback<G> {
    type Head = G::Head;
    type Target = <G::Target as StageTrait>::Fallback;
}

struct Loop;
impl StageTrait for Loop {
    type Fallback = Loop;
}

struct MyStage<T: StageTrait>(T);

impl<T: StageTrait> StageTrait for MyStage<T> {
    type Fallback = T;

    fn run<G: StageFinder<Target = Self>>(head: &mut G::Head) {
        StageTrait::run::<FindFallback<G>>(head);
    }
}

fn main() {
    StageTrait::run::<FindHead<_>>(&mut MyStage(Loop));
}
