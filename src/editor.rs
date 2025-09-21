#[derive(Clone, Debug, PartialEq)]
pub enum InputKind {
    Label,
}

#[derive(Debug)]
pub struct Editor {
    editing: bool,
    input_kind: Option<InputKind>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor { editing: false, input_kind: None, }
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn input_kind(&self) -> Option<InputKind> {
        self.input_kind.clone()
    }

    pub fn begin_input(&mut self, kind: InputKind) {
        self.editing = true;
        self.input_kind = Some(kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initially_not_editing() {
        let editor = Editor::new();
        assert!(!editor.editing());
        assert_eq!(None, editor.input_kind());

    }

    #[test]
    fn after_begin_input_edting_is_true() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.editing());
        assert_eq!(Some(InputKind::Label), editor.input_kind());
    }
}
