// run-pass


// issues #10618 and #16382
// pretty-expanded FIXME #23616

const SIZE: isize = 25;

fn main() {
    let _a: [bool; 1 as usize];
    let _b: [isize; SIZE as usize] = [25; SIZE as usize];
    let _c: [bool; '\n' as usize] = [true; '\n' as usize];
    let _d: [bool; true as usize] = [true; true as usize];
}
