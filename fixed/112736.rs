trait Structure<E>: Sized {
    type RefTarget: ?Sized;
    type FfiPtr;
    unsafe fn borrow_from_ffi_ptr<'a>(ptr: Self::FfiPtr) -> Option<&'a Self::RefTarget>;
}

enum Slice {}

impl<E> Structure<E> for Slice
where
    S: Structure<E>,
    E: Encoding,
{
    type RefTarget = [E::Unit];
    type FfiPtr = (*const E::FfiUnit, usize);
}

trait Encoding {
    type Unit: Unit;
    type FfiUnit;
}

trait Unit {}

enum Utf16 {}

impl Encoding for Utf16 {
    type Unit = Utf16Unit;
    type FfiUnit = u16;
}

struct Utf16Unit(pub u16);

struct SUtf16Str {
    _data: <Slice as Structure<Utf16>>::RefTarget,
}

impl SUtf16Str {
    pub unsafe fn from_ptr<'a>(ptr: <Slice as Structure<Utf16>>::FfiPtr) -> Option<&'a Self> {
        std::mem::transmute::<Option<&[<Utf16 as Encoding>::Unit]>, _>(<Slice as Structure<
            Utf16,
        >>::borrow_from_ffi_ptr(
            ptr
        ))
    }
}
