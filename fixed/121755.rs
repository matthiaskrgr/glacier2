macro x(
    $macro_name:ident,
    $macro2_name:ident,
    $type_name:ident,
    $field_name:ident,
    $const_name:ident
) {
    pub struct $type_name {}

    pub const $const_name: $type_name = $type_name {};

    #[macro_export]
    macro_rules! $macro_name {
        (check_fields) => {{
            assert_eq!($const_name.field, Field::MacroCtxt);
        }};
    }

    pub macro $macro2_name {
            (Copy $e:expr) => {},
            (check_fields) => {test_fields!(check_fields);},

        }
}

x!(test_fields, test_fields2, MyStruct, field, MY_CONST);

pub fn check_fields_local() {
    test_fields!(check_fields);
    test_fields2!(check_fields);
}
