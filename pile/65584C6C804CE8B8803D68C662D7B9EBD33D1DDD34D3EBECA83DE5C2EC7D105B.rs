// ignore-wasm32 compiled with panic=abort by default
// unit-test: ConstProp
// compile-flags: -Zmir-opt-level=1

trait NeedsDrop: Sized {
    const NEEDS: bool = std::mem::needs_drop::<Self>();
}

impl<This> NeedsDrop for This {}

// EMIT_MIR control_flow_simplification.hello.ConstProp.diff
// EMIT_MIR control_flow_simplification.hello.PreCodegen.before.mir
fn hello<T>(){
        panic!()
    }

pub fn main() {
    hello::<()>();
    hello::<Vec<()>>();
}
