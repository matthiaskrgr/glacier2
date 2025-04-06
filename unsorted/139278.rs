struct Foo;

impl Drop for Foo {
    thread_local!(static SPLOK: u32 = 0);
}

thread_local!(static Box: Foo = Foo);
