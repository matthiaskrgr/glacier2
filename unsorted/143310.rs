#![feature(loop_match)]

#[unsafe(no_mangle)]
pub fn break_on_constant_evalution() -> i32 {
    let mut state = 0;
    #[loop_match]
    'a : loop {
        state = 'blk: {
            match state {
                0 => {
                    break 'a;
                },
                _ => {
                    const A: i32 = 0;
                    #[const_continue]
                    break 'blk /* constant evaluation */;
                }
            }
        }
    }
    state
}
