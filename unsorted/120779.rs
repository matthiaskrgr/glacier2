fn range_shadow_lit() {
    let x = 0;

    assert_eq!(2, match val() {
        1..=2 if false =|value_ref, item| 1,
        1 => 2,
        _ => 3
    });

    // ditto
}
