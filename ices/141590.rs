#![feature(never_patterns)]
#![allow(incomplete_features)]

use std::sync::OnceLock;

enum Void {}

static LAZY_INIT: Void = OnceLock::new();
static LAZY_INIT_REF: &[&Void] = &[&LAZY_INIT];

fn main() {}
