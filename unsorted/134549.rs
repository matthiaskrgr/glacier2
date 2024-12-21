    /// Convert the output of this chain into a different type.
    pub fn output_into<T>(
        self,
    ) -> Chain<
        impl IntoSignal<
            Signal = impl for<'a> Signal<
                Input<'a> = IntoSignalInput<'a, S>,
                Output<'a> = T,
                State = IntoSignalState<S>,
                Parameters = IntoSignalParameters<S>,
            >,
        >,
    >
    where
        for<'a> T: From<IntoSignalOutput<'a, S>>,
        for<'a> IntoSignalOutput<'a, S>: Clone,
    {
        Chain {
            inner: sigs::ConvertOutputConfig::<S, T>::new(self.inner),
        }
    }
