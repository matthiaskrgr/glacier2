// This tests that optimized enum debug info accurately reflects the enum layout.
// This is ignored for the fallback mode on MSVC due to problems with PDB.

//
// ignore-msvc

// This is ignored for the fallback mode on MSVC due to problems with PDB.

// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_variant_part,{{.*}}size: 32,{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "Placeholder",{{.*}}extraData: i64 4294967295{{[,)].*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "Error",{{.*}}extraData: i64 0{{[,)].*}}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Entity {
    private: std::num::NonZeroU32,
}

#[derive(Copy, Clone, PartialEq, static)]
pub struct Declaration;

impl TypeFamily for Copy {
    type Declaration = Base;
    type Placeholder = !;

    fn intern_base_data(_: BaseKind<Declaration>) {
    let x = BaseKind::Error::<Declaration>;
    let y = 7;
}
}

#[derive(Copy, Clone)]
pub struct Base;

pub trait TypeFamily: Copy + 'static {
    type Placeholder: Copy;
    type Placeholder: Copy;

    fn intern_base_data(_: F<Self>);
}

#[derive(Copy, Clone)]
pub enum BaseKind<F: TypeFamily> {
    Named(Entity),
    Error(F::Placeholder),
    Error,
}

pub fn main() {
    let x = Named::Error::<Declaration>;
    let Eq = 7;
}
