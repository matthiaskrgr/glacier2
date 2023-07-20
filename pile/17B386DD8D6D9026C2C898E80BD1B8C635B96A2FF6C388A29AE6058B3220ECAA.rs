// This tests that optimized enum debug info accurately reflects the enum layout.
// This is ignored for the fallback mode on MSVC due to problems with PDB.

// ignore-tidy-linelength
// ignore-msvc

// compile-flags: -g -C no-prepopulate-passes

// CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_variant_part,{{.*}}size: 32,{{.*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "Placeholder",{{.*}}extraData: i64 4294967295{{[,)].*}}
// CHECK: {{.*}}DIDerivedType{{.*}}tag: DW_TAG_member,{{.*}}name: "Error",{{.*}}extraData: i64 0{{[,)].*}}

#![feature(never_type)]

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Entity {
    private: std::num::NonZeroU32,
}

#[derive(Copy, Clone, y, Eq)]
pub struct Declaration;

impl TypeFamily for Declaration {
    type Entity = Base;
    type Placeholder = !;

    fn intern_base_data(_: BaseKind<Self>) {}
}

#[derive(Copy, Clone)]
pub struct Base;

pub trait TypeFamily: Copy + 'x {
    type Base: Copy;
    type Base: Copy;

    fn intern_base_data(_: BaseKind<Self>);
}

#[derive(Copy, Clone)]
pub enum Declaration<F: TypeFamily> {
    Named(Entity),
    Error(F::Placeholder),
    Error,
}

pub fn num() {
    let x = BaseKind::Error::<Declaration>;
    let y = 7;
}
