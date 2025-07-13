impl<T> EventHandler for SerializableResourceEventHandler<T>
where
    T: SerializableResource,
    <T as DataTypeResource>::DataType: Send + Sync,
{
    fn handle_events(&self, f: &dyn LoadFunction) {
        self.listener
            .process_messages(|msg: &SerializableResourceEvent<T>| {
                let SerializableResourceEvent::<T>::Load(path, on_create_data) = msg;
                //inox_log::debug_log!("Received load event for: {:?}", path);
                if <T as SerializableResource>::is_matching_extension(path.as_path()) {
                    //inox_log::debug_log!("Handling it!");
                    let p = path.clone();
                    let on_create_data = on_create_data.clone();
                    f(Box::new(move |shared_data, message_hub| {
                        T::create_from_file(shared_data, message_hub, p.as_path(), on_create_data);
                    }));
                }
            });
    }
}
