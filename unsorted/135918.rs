trait FromResidual<R = <Self as Try>::Residual> {}

trait Try {
    type Residual;
}

fn w<U>() {
    let b: &dyn FromResidual = &();
}

fn main() {}
