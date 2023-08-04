#![feature(rustc_private)]

extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;

extern crate rustc_driver;

use rustc_session::parse::ParseSess;
use rustc_span::source_map::FilePathMapping;

fn main() {
    rustc_span::create_default_session_globals_then(|| run());
}

fn run() {
    let ps = ParseSess::new(
        vec![rustc_parse::DEFAULT_LOCALE_RESOURCE],
        FilePathMapping::empty(),
    );
}
