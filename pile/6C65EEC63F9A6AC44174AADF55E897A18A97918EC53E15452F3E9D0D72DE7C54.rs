// check-pass

#[doc(include = "external-cross-doc.md")]
//~^ WARNING unknown `doc` attribute `include`
//~| HELP use `doc = include_str!` instead
// FIXME(#85497): make this a deny instead so it's more clear what's happening
//~| NOTE see issue #82730
//~| WARNING previously accepted
//~| NOTE see issue #82730
pub struct NeedMoreDocs;
