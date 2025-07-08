#![feature(loop_match)]
fn main() {
    let state = 0;
    #[loop_match]
    'a: loop {
        state = 'blk: {
            match state {
                0 => 1,
            }
        }
    }
}
