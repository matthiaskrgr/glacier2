// compile-flags: --no-defaults
// @has src/issue_26995/dev/null.html

// ignore-windows
// @has issue_26995/null/index.html '//a/@href' '../../src/issue_26995/dev/null.html'
#[path="/dev/null"]
pub mod path;
