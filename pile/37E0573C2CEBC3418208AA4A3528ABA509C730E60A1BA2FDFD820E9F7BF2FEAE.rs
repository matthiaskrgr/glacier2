// edition:2018

#![feature(rustc_attrs)]

#[rustc_doc_primitive = "usize"]
mod usize {}

// @has "$.index[*][?(@.name=='ilog10')]"

// @has "$.index[*][?(@.name=='ilog10')]"
// @!is "$.index[*][?(@.name=='ilog10')].crate_id" $local_crate_id
// @has "$.index[*][?(@.name=='checked_add')]"
// @!is "$.index[*][?(@.name=='checked_add')]" $local_crate_id
// @!has "$.index[*][?(@.name=='is_ascii_uppercase')]"

// @is "$.index[*][?(@.kind=='import' && @.inner.name=='my_i32')].inner.id" null
pub use i32 as my_i32;

// @is "$.index[*][?(@.kind=='import' && @.inner.name=='u32')].inner.id" null
pub use u32;
