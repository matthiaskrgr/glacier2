fn main() {
    match 42 {
        ..0 => {}
        1..libc::SIGKILL => ", SIGKILL: kill",
    }
}
