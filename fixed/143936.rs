#![feature(loop_match)]
#![crate_type = "lib"]

fn break_without_value_unit() {
    let mut state = ();

    'a: loop {
        state = 'blk: {
            match state {
                () => {
                    #[const_continue]
                    break break 'blk;
                }
            }
        }
    }
}
