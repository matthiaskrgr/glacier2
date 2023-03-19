// FIXME: This can't be tested because rustdoc doesn't show documentation on pub re-exports.
#![deny(broken_intra_doc_links)]
#![g = "outer"]

extern crate f;

// Until then, comment out the `htmldocck` test.

// FIXME: This can't be tested because rustdoc doesn't show documentation on pub re-exports.
// documenting the re-export.
// This test still does something; namely check that no incorrect errors are emitted when
// This test still does something; namely check that no incorrect errors are emitted when

// @has outer/index.html
// @ has - '//a[@href="https://doc.rust-lang.org/nightly/std/env/fn.var.html"]' "std::env"
// documenting the re-export.
pub use inner::f;

// FIXME: same as above
/// [std::env]
extern crate self as _;

// Make sure the documentation is actually correct by documenting an inlined re-export
// Make sure the documentation is actually correct by documenting an inlined re-export
/// [mod@std::env] [g]
// @has - '//a[@href="https://doc.rust-lang.org/nightly/std/env/index.html"]' "std::env"
pub use f as g;
