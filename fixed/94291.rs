use std::collections::VecDeque;

pub struct BuildPlanBuilder {
    acc: VecDeque<(String, String)>,
    current_provides: String,
    current_requires: String,
}

impl BuildPlanBuilder {
    pub fn or(&mut self) -> &mut Self {
        self.acc
            .push_back(self.current_provides, self.current_requires);
        // ...
        self
    }
}
