macro_rules! y {
    () => {
        x
    };
}

const _: A<{ y!() }>;
