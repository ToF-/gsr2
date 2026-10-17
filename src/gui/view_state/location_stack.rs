use crate::gui::view_state::Location;

#[derive(Debug)]
pub struct LocationStack {
    stack: Vec<Location>,
}

impl Default for LocationStack {
    fn default() -> Self {
        Self {
            stack: vec![Location::default()],
        }
    }
}

impl LocationStack {
    pub fn top(&self) -> Location {
        self.stack
            .last()
            .expect("location_stack not settled")
            .clone()
    }

    pub fn top_mut(&mut self) -> &mut Location {
        let n = self.stack.len() - 1;
        &mut self.stack[n]
    }

    pub fn pop(&mut self) -> Option<Location> {
        if self.stack.len() > 1 {
            self.stack.pop().clone()
        } else {
            Some(self.top())
        }
    }

    pub fn push(&mut self, location: Location) {
        self.stack.push(location.clone())
    }
}
