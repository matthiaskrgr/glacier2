struct CompileError;

impl Drop for CompileError {
    fn drop(&mut self) {}
}

impl CompileError {
    fn throw(&self) {}
}

fn main() {
    'code: {
        break 'code;
        CompileError.throw()
 
   };
}
