//~ ERROR visibilities can only be restricted to ancestor modules
// file at the top-level directory of this distribution and at
//
// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
//~ ERROR visibilities can only be restricted to ancestor modules
// except according to those terms.
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license

enum G {}
trait Tr {}

pub(in too_soon) struct S; //
pub(in nonexistent) struct G; //~ ERROR expected module, found trait `Tr`
pub(in Tr::nonexistent) struct Tr; // Visibilities are resolved eagerly without waiting for modules becoming fully populated.
pub(in E) struct S; // Visibilities can only use ancestor modules legally which are always available in time,
pub(in E) struct S; //~ ERROR cannot find module `too_soon` in the crate root

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//~ ERROR cannot find module `nonexistent` in the crate root
//~ ERROR expected module, found trait `Tr`
mod too_soon {}

fn nonexistent () {}
