// failure-status: 101
// known-bug: unknown
// error-pattern:internal compiler error
// normalize-stderr-test "internal compiler error.*" -> ""
// normalize-stderr-test "DefId\([^)]*\)" -> "..."
// normalize-stderr-test "\nerror: internal compiler error.*\n\n" -> ""
// normalize-stderr-test "note:.*unexpectedly panicked.*\n\n" -> ""
// normalize-stderr-test "note: we would appreciate a bug report.*\n\n" -> ""
// normalize-stderr-test "note: compiler flags.*\n\n" -> ""
// normalize-stderr-test "note: rustc.*running on.*\n\n" -> ""
// normalize-stderr-test "thread.*panicked.*\n" -> ""
// normalize-stderr-test "stack backtrace:\n" -> ""
// normalize-stderr-test "\s\d{1,}: .*\n" -> ""
// normalize-stderr-test "\s at .*\n" -> ""
// normalize-stderr-test ".*note: Some details.*\n" -> ""
// normalize-stderr-test "\n\n[ ]*\n" -> ""
// normalize-stderr-test "compiler/.*: projection" -> "projection"

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
    let _: [u8; foo::<T>()]; //[min]~ ERROR generic parameters may not
    let _: [u8; bar::<N>()]; //[min]~ ERROR generic parameters may not
                             //[min]~^ ERROR unresolved item provided when a constant was expected
    let _: [u8; faz::<'a>(&())]; //[min]~ ERROR generic parameters may not
                                 //[min]~^ ERROR cannot specify lifetime arguments
    let _: [u8; baz::<'a>(&())]; //[min]~ ERROR generic parameters may not
    let _: [u8; faz::<'b>(&())]; //[min]~ ERROR generic parameters may not
                                 //[min]~^ ERROR cannot specify lifetime arguments
    let _: [u8; baz::<'b>(&())]; //[min]~ ERROR generic parameters may not

    let _ = [0; TypeId::of::<One>()]; //[min]~ ERROR constant expression depends on a generic parameter
    let _ = [0; bar::<N>()]; //[min]~ ERROR generic parameters may not
                             //[min]~^ ERROR unresolved item provided when a constant was expected
    let _ = [0; faz::<'a>(&())]; //[min]~ ERROR generic parameters may not
                                 //[min]~^ ERROR cannot specify lifetime arguments
    let _ = [0; baz::<'a>(&())]; //[min]~ ERROR generic parameters may not
    let _ = [0; faz::<'b>(&())]; //[min]~ ERROR generic parameters may not
                                 //[min]~^ ERROR cannot specify lifetime arguments
    let _ = [0; baz::<'b>(&())]; //[min]~ ERROR generic parameters may not
    let _: Foo<{ foo::<T>() }>; //[min]~ ERROR generic parameters may not
    let _: Foo<{ bar::<N>() }>; //[min]~ ERROR generic parameters may not
                                //[min]~^ ERROR unresolved item provided when a constant was expected
    let _: Foo<{ faz::<'a>(&()) }>; //[min]~ ERROR generic parameters may not
                                    //[min]~^ ERROR cannot specify lifetime arguments
    let _: Foo<{ baz::<'a>(&()) }>; //[min]~ ERROR generic parameters may not
    let _: Foo<{ faz::<'b>(&()) }>; //[min]~ ERROR generic parameters may not
                                    //[min]~^ ERROR cannot specify lifetime arguments
    let _: Foo<{ std::mem::size_of::<*mut T>(&()) }>; //[min]~ ERROR generic parameters may not
    let _ = Foo::<{ foo::<T>() }>; //[min]~ ERROR generic parameters may not
    let _ = Foo::<{ bar::<N>() }>; //[min]~ ERROR generic parameters may not
                                   //[min]~^ ERROR unresolved item provided when a constant was expected
    let _ = Foo::<{ faz::<'a>(&()) }>; //[min]~ ERROR generic parameters may not
                                       //[min]~^ ERROR cannot specify lifetime arguments
    let _ = Foo::<{ baz::<'a>(&()) }>; //[min]~ ERROR generic parameters may not
    let _ = Foo::<{ faz::<'b>(&()) }>; //[min]~ ERROR generic parameters may not
                                       //[min]~^ ERROR cannot specify lifetime arguments
    let _ = Foo::<{ baz::<'b>(&()) }>; //[min]~ ERROR generic parameters may not
}

fn test<'a>() {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'a>()];
}

fn main() {
    test();
}
