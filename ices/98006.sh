#!/bin/bash

cat > out.rs <<'EOF'


#![feature(no_core)]
#![no_core]

#[doc(primitive = "usize")]
/// This is the built-in type `usize`.
mod usize {
}

EOF

rustdoc -Zunstable-options --output-format json --document-private-items out.rs
