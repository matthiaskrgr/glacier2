trait Project {
    const SELF: Self;
}

fn take1(_: Project<SELF = { loop {} }>) {}
