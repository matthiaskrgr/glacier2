fn op_import_sync<'s>(scope: &mut v8::HandleScope<'s>) {
  let source = v8::String::new(scope, "").unwrap();
  let origin = v8::ScriptOrigin::new(scope, source.into(), 0, 0, false, 0, None, true, false, true, None);
  let mut source = v8::script_compiler::Source::new(source, Some(&origin));
  let wrapper_module = v8::script_compiler::compile_module(scope, &mut source).unwrap();
  const X: std::sync::Mutex<Option<v8::Global<v8::Module>>> = std::sync::Mutex::new(None);
  wrapper_module
    .instantiate_module(scope, |context: v8::Local<v8::Context>, _, _, _| {
      let scope = unsafe { v8::CallbackScope::new(context) };
      Some(v8::Local::new(
        &mut scope,
        X.lock().unwrap().take().unwrap(),
      ))
    })
    .unwrap();
}

fn main() {}
