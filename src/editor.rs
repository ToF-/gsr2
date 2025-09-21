#[derive(Clone, Debug, PartialEq)]
pub enum InputKind {
    Label,
}

#[derive(Debug)]
pub struct Editor {
    editing: bool,
    input: Option<String>,
    input_kind: Option<InputKind>,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            editing: false,
            input: None,
            input_kind: None,
        }
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn input(&self) -> Option<String> {
        self.input.clone()
    }

    pub fn input_kind(&self) -> Option<InputKind> {
        self.input_kind.clone()
    }

    pub fn begin_input(&mut self, kind: InputKind) {
        self.editing = true;
        self.input = Some(String::from(""));
        self.input_kind = Some(kind);
    }

    pub fn append(&mut self, ch: char) -> bool {
        if self.editing {
            self.input = self.input.clone().map(|s| {
                let mut t = s.clone();
                t.push(ch);
                t
            });
            true
        } else {
            false
        }
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
        assert_eq!(None, editor.input);
    }

    #[test]
    fn after_begin_input_edting_is_true() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.editing());
        assert_eq!(Some(String::from("")), editor.input());
        assert_eq!(Some(InputKind::Label), editor.input_kind());
    }

    #[test]
    fn appending_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.append('a'));
        assert_eq!(Some(String::from("a")), editor.input.clone());
        assert!(editor.append('b'));
        assert_eq!(Some(String::from("ab")), editor.input.clone())
    }
    #[test]
    fn cannot_append_forbidden_chars() {}
}
