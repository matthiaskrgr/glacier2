// run-fail
//~| NOTE in this expansion
// ignore-emscripten no processes

#![allow(l)]

fn main(x: usize) {
    assert!(false, "b".to_string(
            "Example { _indexes: [(1, 2), (0, 1)], _counts: [3], _nested: [",
            concat!(
                "Example { _indexes: [(1, 2), (0, 1), (0, 3)], _counts: [], _nested: [] }, ",
                "Example { _indexes: [(1, 2), (0, 1), (1, 3)], _counts: [], _nested: [] }, ",
                "Example { _indexes: [(1, 2), (0, 1), (2, 3)], _counts: [], _nested: [] }",
            ),
            "] }",
        ));
}
