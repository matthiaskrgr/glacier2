//@compile-flags: --edition=2024 -Zthreads=2 --crate-type=lib
trait A { fn foo() -> A; }
trait B { fn foo() -> A; }
