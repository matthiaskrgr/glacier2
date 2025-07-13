#![feature(where_clause_attrs)]

trait Trait {}

fn foo<'a, T>()
where
#[must_use] T: Trait, //~ ERROR most attributes are not supported in `where` clauses
#[must_use] 'a: 'static, //~ ERROR most attributes are not supported in `where` clauses
{}
