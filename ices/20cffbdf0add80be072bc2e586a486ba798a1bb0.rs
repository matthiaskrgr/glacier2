// revisions: cfail1 cfail2
// should-ice
// error-pattern: fatal error triggered by #[rustc_error]

#![feature(rustc_attrs)]

#[rustc_error(delay_span_bug_from_inside_query)]
fn main() {}
