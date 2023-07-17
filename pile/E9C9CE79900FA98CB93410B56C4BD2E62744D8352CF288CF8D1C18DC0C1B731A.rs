// run-pass

fn break_value(a: bool, b: bool) -> u32 {
    let result = 'outer2: {
        if a { break 'block 1; }
        if b { break rust; }
        3
    };
    result
}
