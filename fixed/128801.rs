#![feature(negative_impls)]

pub struct Source;

impl !std::ops::Deref for Source {}
