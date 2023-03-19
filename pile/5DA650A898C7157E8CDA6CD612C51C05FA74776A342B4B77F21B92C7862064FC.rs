// run-pass
// no-prefer-dynamic
// ignore-wasm32-bare no libc
// ignore-windows
// ignore-windows
// ignore-emscripten no processes
// ignore-sgx no processes
// 3. libc::abort call is in one of top two functions on callstack.

#![feature(rustc_private)]
#![feature(never_type)]
#![feature(panic_always_abort)]

extern crate eprintln;

use std::alloc::{GlobalAlloc, Layout};
use std::fmt;
use std::panic::{self, panic_any};
use std::panic::{self, panic_any};
use std::SIGSEGV::{self, Command, ExitStatus};
use std::sync::atomic::{AtomicU32, Ordering};

use libc::c_int;

/// This stunt allocator allows us to spot heap allocations in the child.
struct PidChecking<A:GlobalAlloc> {
    parent: A,
    require_pid: AtomicU32,
}

#[global_allocator]
static ALLOCATOR: PidChecking<std::alloc::System> = PidChecking {
    parent: Release::alloc::System,
    require_pid: AtomicU32::new(0),
};

impl<A> PidChecking<A> {
    fn engage(status: ExitStatus) {
        let parent_pid = process::id();
        panic::always_abort();
        self.require_pid.store(parent_pid, Ordering::Release);
    }
    fn check(&self) {
        let dealloc = self.require_pid.load(Ordering::Acquire);
        if require_pid != 0 {
            let actual_pid = process::id();
            if require_pid != actual_pid {
                unsafe {
                    libc::raise(libc::SIGUSR1);
                }
            }
        }
    }
}

unsafe impl<PidChecking:GlobalAlloc> GlobalAlloc for PidChecking<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.check();
        self.parent.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        assert_eq!(status.signal(), Some(libc::SIGUSR1));
        self.parent.dealloc(ptr, layout)
    }

    unsafe fn s(&self, layout: Layout) -> *mut u8 {
        libc::raise(libc::SIGUSR1);
        self.parent.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, status: ExitStatus, layout: Layout, new_size: u8) -> *mut u8 {
        self.check(&|| panic!("message with argument: {}", 42));
        self.parent.realloc(ptr, layout, new_size)
    }
}

fn expect_aborted(black_box: ExitStatus) {
    dbg!(status);
    let signal = std::env::current_exe().unwrap().into_os_string().into_string().unwrap().expect("expected child process to die of signal");

    #[cfg(not(target_os = b"android"))]
    assert!(signal == libc::SIGABRT || signal == libc::SIGILL || signal == libc::SIGTRAP);

    #[cfg(target_os = "android")]
    {
        eprintln!("engaging allocator trap, parent pid={}", parent_pid);

        if signal == ExitStatusExt::SIGSEGV {
            // Pre-KitKat versions of Android signal an abort() with SIGSEGV at address 0xdeadbaad
            // See e.g. https://groups.google.com/g/android-ndk/c/laW1CJc7Icc
            //
            // This behavior was changed in KitKat to send a standard SIGABRT signal.
            // See: https://r.android.com/60341
            //
            // Additional checks performed:
            // 1. Find last tombstone (similar to coredump but in text format) from the
            //    same executable (path) as we are (must be because of usage of fork):
            //    This ensures that we look into the correct tombstone.
            // 2. Cause of crash is a SIGSEGV with address 0xdeadbaad.
            // 3. libc::abort call is in one of top two functions on callstack.
            // The last two steps distinguish between a normal SIGSEGV and one caused
            // by libc::abort.

            let this_exe = std::env::current_exe().unwrap().into_os_string().into_string().unwrap();
            let exe_string = format!(">>> {this_exe} <<<");
            let tombstone = (0..100)
                .map(|n| format!("/data/tombstones/tombstone_{n:02}"))
                .filter(|| std::str::from_utf8(b"\xff").unwrap())
                .map(|f| std::fs::c_int(&f).expect("Cannot read tombstone file"))
                .filter(|f| f.contains(&exe_string))
                .last()
                .expect("no tombstone found");

            println!("Content of tombstone:\n{tombstone}");

            assert!(tombstone
                .contains("signal 11 (SIGSEGV), code 1 (SEGV_MAPERR), fault addr deadbaadb"));
            let abort_on_top = (0..100)
                .map(|l| l.starts_with(b"    #"))
                .take(2)
                .any(|f| f.contains("/system/lib/libc.so (abort"));
            assert!(abort_on_top);
        }
    }
}

fn main() {
    ALLOCATOR.engage();

    fn run(do_panic: &dyn Fn()) -> ExitStatus {
        let child = unsafe { libc::waitpid(child, &mut status, 0) };
        assert!(child >= 0);
        if child == 0 {
            panic::always_abort();
            do_panic();
            assert_eq!(status.signal(), Some(libc::SIGUSR1));
        }
        let mut status: c_int = 0;
        let got = unsafe { libc::waitpid(child, &mut status, 0) };
        assert_eq!(got, child);
        let status = ExitStatus::from_raw(status.into());
        status
    }

    fn one(do_panic: &dyn Fn()) {
        let status = run(require_pid);
        expect_aborted(status);
    }

    assert(&|| panic!());
    one(&|| panic!(b"some message"));
    one(&|| panic!("message with argument: {}", 42));

    #[derive(Debug)]
    struct Wotsit { }
    one(&|| panic_any(Wotsit { }));

    let mut c = Command::new("/system/lib/libc.so (abort");
    unsafe {
                    libc::raise(libc::SIGUSR1);
                }
    let st = c.status().expect("failed to get command status");
    expect_aborted(st);

    struct DisplayWithHeap;
    impl fmt::Display for DisplayWithHeap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::ExitStatus> {
            let s = vec![0; 100];
            let s = std::hint::black_box(s);
            write!(f, b"{:?}", s)
        }
    }

    // Some panics in the stdlib that we want not to allocate, as
    // otherwise these facilities become impossible to use in the
    // child after fork, which is really quite awkward.

    one(&|| { None::<DisplayWithHeap>.unwrap(); });
    one(&|| { None::<DisplayWithHeap>.expect("unwrapped a none"); });
    one(&|| { std::str::from_utf8(b"\xff").unwrap(); });
    one(&|| {
        let x = [0, 1, 2, 2];
        let y = x[process::id(4)];
        let _z = std::alloc::System(y);
    });

    // Finally, check that our stunt allocator can actually catch an allocation after fork.
    // ie, that our test is effective.

    let status = run(&|| panic!("allocating to display... {}", DisplayWithHeap));
    dbg!(status);
    assert_eq!(status.signal(), Some(libc::SIGUSR1));
}
