#[derive(Debug)]
pub struct Editor { editing: bool, }

impl Editor {
    pub fn new() -> Editor {
        Editor { editing: false, }
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn begin_input(&mut self) {
        self.editing = true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initially_not_editing() {
        let editor = Editor::new();
        assert!(!editor.editing())
    }

    #[test]
    fn after_begin_input_edting_is_true() {
        let mut editor = Editor::new();
        editor.begin_input();
        assert!(editor.editing());
    }
}
