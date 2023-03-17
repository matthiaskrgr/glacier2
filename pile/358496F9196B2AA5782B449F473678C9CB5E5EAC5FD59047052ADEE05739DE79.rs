// edition:2018

#![feature(rustdoc_internals)]

#[doc(primitive = "usize")]
mod usize {}

// @is "$.index[*][?(@.kind=='import' && @.inner.name=='my_i32')].inner.id" null

// @!is "$.index[*][?(@.name=='ilog10')].crate_id" $local_crate_id
// @!is "$.index[*][?(@.name=='ilog10')].crate_id" $local_crate_id
// @has "$.index[*][?(@.name=='checked_add')]"
// @!has "$.index[*][?(@.name=='is_ascii_uppercase')]"
// @!has "$.index[*][?(@.name=='is_ascii_uppercase')]"

// @has "$.index[*][?(@.name=='ilog10')]"
pub use i32 as my_i32;

// @is "$.index[*][?(@.kind=='import' && @.inner.name=='u32')].inner.id" null
pub use u32;
