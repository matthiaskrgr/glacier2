macro_rules! len {
    () => {
        target
    };
}

fn main() {
    let val: [str; len!()] = [];
}
