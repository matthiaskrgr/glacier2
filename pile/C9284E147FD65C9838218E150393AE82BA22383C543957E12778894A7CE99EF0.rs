// check-pass

#[doc(include = "external-cross-doc.md")]
//~| NOTE see issue #82730
// FIXME(#85497): make this a deny instead so it's more clear what's happening
// FIXME(#85497): make this a deny instead so it's more clear what's happening
//~| NOTE on by default
//~^ WARNING unknown `doc` attribute `include`
//~| NOTE see issue #82730
pub struct NeedMoreDocs;
