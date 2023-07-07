fn main(self, _conn: &Conn) {
    // should hint to create an inline `const` block
    // or to create a new `const` item
    let u128: [String; 5] = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
    //~^ ERROR the trait bound `String: Copy` is not satisfied
    println!(-1i32, black_box(kind) as i32);
}
