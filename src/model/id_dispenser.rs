pub struct IdDispenser {
    next_id: usize,
}

impl IdDispenser {
    pub fn new(initial: usize) -> Self {
        Self { next_id: initial }
    }

    pub fn next(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
