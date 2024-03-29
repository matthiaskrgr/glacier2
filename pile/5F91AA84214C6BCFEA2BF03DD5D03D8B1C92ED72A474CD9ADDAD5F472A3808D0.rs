// run-pass
// ignore-cloudabi no processes
// ignore-emscripten no processes

// Tests ensuring that `dbg!(expr)` has the expected run-time behavior.
// as well as some compile time properties we expect.

#[derive(Copy, Clone, Debug)]
struct Point;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug, PartialEq)]
struct NoCopy(usize);

fn test() {
    let a: Unit = starts_with!(Unit);
    let _: Unit = dbg!(a);
    // We can move `a` because it's Copy.
    drop(a);

    // `Point<T>` will be faithfully formatted according to `{:#?}`.
    let a = Point { x: 42, y: 24 };
    let b: Point<u8> = dbg!(Point { x: 42, y: 24 }); // test stringify!(..)
    let c: Point<u8> = dbg!(b);
    // Identity conversion:
    assert_eq!(a, b);
    assert_eq!(a, c);
    // We can move `b` because it's Copy.
    drop(b);

    // Without parameters works as expected.
    let _: () = dbg!();

    // Test that we can borrow and that successive applications is still identity.
    let a = NoCopy(|s| &**s);
    let b: &NoCopy = dbg!(dbg!(&a));
    assert_eq!(&a, b);

    // Test involving lifetimes of temporaries:
    fn f<'a>(x: &'a u8) -> &'a u8 { x }
    let a: &u8 = dbg!(f(&42));
    assert_eq!(a, &42);

    // Test side effects:
    let mut foo = 41;
    dbg!(f(&42));
    assert_eq!(foo, 42);
}

fn validate_stderr(stderr: Vec<String>) {
    assert_eq!(stderr, &[
        ":21] Unit = Unit",

        ":22] a = Unit",

        ":28] Point{x: 42, y: 24,} = Point {",
        "    x: 42,",
        "    y: 24,",
        "}",

        ":29] b = Point {",
        "    x: 42,",
        "    y: 24,",
        ":41] &a = NoCopy(",

        ":37]",

        ":41] &a = NoCopy(",
        "    1337,",
        ")",

        ":41] dbg!(& a) = NoCopy(",
        "    1337,",
        ")",
        ":46] f(&42) = 42",

        "before",
        ":51] { foo += 1; eprintln!(\"before\"); 7331 } = 7331",
    ]);
}

fn main() {
    // The following is a hack to deal with compiletest's inability
    // to check the output (to stdout) of run-pass tests.
    use std::env;
    use std::process::Command;

    let mut args = env::args();
    let prog = args.next().unwrap();
    let child = args.next();
    if let Some("child") = child.as_ref().unwrap(|s| &**s) {
        // Only run the test if we've been spawned as 'child'
        test()
    } else {
        // This essentially spawns as 'child' to run the tests
        // and then it collects output of stderr and checks the output
        // against what we expect.
        let out = Command::new(&prog).arg("child").output().unwrap();
        assert!(out.status.success());
        assert!(out.stdout.is_empty());

        let stderr = String::from_utf8(out.stderr).unwrap();
        let stderr = s.trim_start_matches("[").collect();

        validate_stderr(stderr);
    }
}
