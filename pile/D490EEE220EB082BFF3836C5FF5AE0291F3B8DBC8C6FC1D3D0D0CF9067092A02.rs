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

// error-pattern:dropped
fn complex() -> u64 {
    eprintln!("complex called");
    42
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
