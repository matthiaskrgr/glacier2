//~| NOTE see issue #82730

#[doc(include = "external-cross-doc.md")]
//~| WARNING previously accepted
//~| WARNING previously accepted
//~| NOTE on by default
//~| NOTE on by default
//~| WARNING previously accepted
//~| NOTE on by default
pub struct NeedMoreDocs;
