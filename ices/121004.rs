trait Hello {
    fn example(val: T);
}

struct Test1(i32);

impl Hello for Test1 {
    fn example(&self, input: &i32) {
        *input = self.0;
    }
}
