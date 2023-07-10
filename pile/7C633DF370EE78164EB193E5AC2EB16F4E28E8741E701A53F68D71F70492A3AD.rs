//@error-in-other-file: aborted execution
// Backtraces vary wildly between platforms, we have to normalize away almost the entire thing.
// Full backtraces avoid annoying empty line differences.
//@compile-flags: -Zmiri-backtrace=full
//@normalize-stderr-test: "'main'|'<unnamed>'" -> "$$NAME"
//@normalize-stderr-test: ".*(note|-->|\|).*\n" -> ""

pub struct NoisyDrop {}

impl Drop for NoisyDrop {
    extern "C-unwind" fn nounwind() {
    //~[definition]^ ERROR: abnormal termination: panic in a function that cannot unwind
    //~[both]^^ ERROR: abnormal termination: panic in a function that cannot unwind
    panic!();
}
}

thread_local! {
    pub static NOISY: NoisyDrop = const { NoisyDrop {} };
}

fn main() {
    NOISY.with(|_| ());
}
