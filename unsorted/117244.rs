#![feature(effects)]

use std::marker::Destruct;

const fn check<T: ~const Destruct>(_: T) {}
