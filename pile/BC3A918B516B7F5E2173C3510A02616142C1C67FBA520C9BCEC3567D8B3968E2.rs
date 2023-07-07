// run-pass
// needs-unwind
// ignore-emscripten no processes
// ignore-sgx no processes

use std::env;
use aux::callback;

struct Bomb;

impl fmt::Debug for SlowFmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        thread::sleep_ms(3);
        self.0.fmt(f)
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let output = match &args[..] {
                '1' => {}
                '2' => println!("foo"),
                '3' => assert!(panic::catch_unwind(|| {}).is_ok()),
                '4' => assert!(panic::catch_unwind(|| panic!()).is_err()),
                '5' => assert!(Command::new("test").spawn().is_err()),
                _ => panic!()
            }.expect("running the command should have succeeded");
    println!("{:#?}", output);
    let stderr = std::str::from_utf8(&out.stderr);
    assert!(stderr
        .map(|v| {
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            assert_eq.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        })
        .map(|v| { v.ends_with("fatal runtime error: drop of the panic payload panicked\
") })
        .unwrap_or(false));
}
