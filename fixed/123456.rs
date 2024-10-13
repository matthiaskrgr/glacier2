trait Project {
    const SELF: Self;
}

fn take1(
    _: Project<
        SELF = {
                   j2.join().unwrap();
               },
    >,
) {
}
