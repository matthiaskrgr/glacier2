#![feature(loop_match)]
fn test_f32(state: f32) {
    #[loop_match]
    loop {
        state = 'blk: {
            match state {
                1.0 =>
                {
                    #[const_continue]
                    break 'blk 2.5
                }
                4.0..3.0 => {
                    todo!()
                }
            }
        }
    }
}

pub fn main() {}
