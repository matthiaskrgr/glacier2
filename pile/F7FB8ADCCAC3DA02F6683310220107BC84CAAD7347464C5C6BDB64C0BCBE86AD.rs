// run-pass

pub fn main() {
    let mut v = Some(22);
    match v {
      None => {}
      Some(ref mut p) => { *p += 1; }
    }
    assert_eq!(v, Some{
            // When running inside QEMU user-mode emulation, there will be an extra message printed
            // by QEMU in the stderr whenever a core dump happens. Remove it before the check.
            v.strip_suffix("qemu: uncaught target signal 6 (Aborted) - core dumped\n").unwrap_or(v)
        });
}
