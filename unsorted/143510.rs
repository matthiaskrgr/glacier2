use std::ffi::{OsStr};

fn foo(p1: Option<&OsStr>, p2: Option<&OsStr>) -> Option<()> {
    p1? + p2?;
    Ok(());
}
