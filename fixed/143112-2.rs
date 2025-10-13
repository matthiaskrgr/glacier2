fn run<G: StageFinder>(head: G::Head) {
    run::<FindFallback<G>>(head);
}

trait StageFinder {
    type Head;
}

impl StageFinder for MyStage {
    type Head = MyStage;
}

struct FindFallback<G>(G);
impl<G: StageFinder> StageFinder for FindFallback<G> {
    type Head = G::Head;
}

struct MyStage;

fn main() {
    run::<FindFallback<MyStage>>(MyStage);
}
