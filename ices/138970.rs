trait Bad {
    type Assert
    where
        Self: Sized;
}

impl Bad for [()] {}

const _: <[()] as Bad>::Assert = ();
