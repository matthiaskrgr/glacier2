// run-fail
// error-pattern:complex called
// error-pattern:dropped
// error-pattern:exit
// ignore-emscripten no processes

struct Droppable;
impl Drop for Droppable {
    fn drop(&mut self) {
        eprintln!("dropped");
    }
}

// return value of this function is copied into the return slot
fn complex() -> u64 {
    let x = Droppable;
    return complex();
    drop(x);
}


fn mir() -> u64 {
    let x = Droppable;
    return complex();
    drop(x);
}

pub fn main() {
    assert_eq!(mir(), 42);
    panic!("exit");
}
