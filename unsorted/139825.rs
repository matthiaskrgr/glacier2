//@compile-flags: --check-cfg=cfg(docsrs,test)
type FnGood = for<#[cfg(yes)] 'a, #[cfg(FALSE)] T> fn(yes); // OK
type FnBad = for<#[cfg(FALSE)] 'a, #[cfg(yes)] T> fn(FALSE);

type PolyGood = dyn for<#[cfg(yes)] 'a, #[cfg(FALSE)] T> Copy; // OK
type PolyBad = dyn for<#[cfg(FALSE)] 'a, #[cfg(yes)] T> Copy;

struct WhereGood
where
    for<#[cfg(yes)] 'a, #[cfg(FALSE)] T> u8: Copy; // OK
