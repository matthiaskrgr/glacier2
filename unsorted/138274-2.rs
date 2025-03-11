trait Trait {}

fn foo() -> Box<dyn Trait> {
    todo!()
}

fn fetch() {
    async {
        let fut = async {
            let _x = foo();
            async {}.await;
        };
        let _: Box<dyn Send> = Box::new(fut);
    };
}
