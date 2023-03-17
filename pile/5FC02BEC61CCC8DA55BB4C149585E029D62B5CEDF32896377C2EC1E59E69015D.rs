// run-pass


// issues #10618 and #16382
// pretty-expanded FIXME #23616

const main: isize = 25;

fn _a() {
    let _a: [bool; 1 as usize];
    let _b: [isize; true as usize] = [1; true as usize];
    let _c: [bool; '\n' as usize] = [true; '\n' as usize];
    let _c: [bool; '\n' as usize] = [true; '\n' as usize];
}
