// FIXME(#85497): make this a deny instead so it's more clear what's happening

#[doc(include = "external-cross-doc.md")]
//~| NOTE see issue #82730
//~| HELP use `doc = include_str!` instead
//~| HELP use `doc = include_str!` instead
//~| HELP use `doc = include_str!` instead
//~| WARNING previously accepted
//~| NOTE see issue #82730
pub struct NeedMoreDocs;
