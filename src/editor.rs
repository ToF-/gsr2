#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum InputKind {
    Label,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Editor {
    editing: bool,
    input: String,
    input_kind: Option<InputKind>,
}

#[allow(dead_code)]
impl Editor {
    pub fn new() -> Editor {
        Editor {
            editing: false,
            input: String::from(""),
            input_kind: None,
        }
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn input_kind(&self) -> Option<InputKind> {
        self.input_kind.clone()
    }

    pub fn begin_input(&mut self, kind: InputKind) {
        self.editing = true;
        self.input = String::from("");
        self.input_kind = Some(kind);
        println!("begin input");
    }

    pub fn confirm_input(&mut self) -> String {
        if self.editing {
            self.editing = false;
            self.input.clone()
        } else {
            String::from("")
        }
    }

    pub fn cancel_input(&mut self) {
        if self.editing {
            self.editing = false;
            self.input = String::from("")
        }
    }
    pub fn append(&mut self, ch: char) -> bool {
        if self.editing && matches!(ch, 'a'..='z' | '0'..='9' | '-' | '_') {
            self.input.push(ch);
            println!("{}", self.input);
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self) -> bool {
        if self.editing && !self.input.clone().is_empty() {
            self.input.pop();
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
    }

    #[test]
    fn after_begin_input_edting_is_true() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.editing());
        assert_eq!(String::from(""), editor.input());
        assert_eq!(Some(InputKind::Label), editor.input_kind());
    }

    #[test]
    fn appending_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.append('a'));
        assert_eq!(String::from("a"), editor.input.clone());
        assert!(editor.append('b'));
        assert_eq!(String::from("ab"), editor.input.clone());
        assert!(editor.append('0'));
        assert_eq!(String::from("ab0"), editor.input.clone());
        assert!(editor.append('9'));
        assert_eq!(String::from("ab09"), editor.input.clone());
        assert!(editor.append('-'));
        assert_eq!(String::from("ab09-"), editor.input.clone());
        assert!(editor.append('_'));
        assert_eq!(String::from("ab09-_"), editor.input.clone());
    }
    #[test]
    fn cannot_append_forbidden_chars() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(!editor.append('"'));
        assert!(!editor.append('@'));
        assert!(!editor.append('^'));
        assert_eq!(String::from(""), editor.input());
    }
    #[test]
    fn deleting_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.append('a'));
        assert!(editor.append('b'));
        assert!(editor.append('c'));
        assert!(editor.delete());
        assert_eq!(String::from("ab"), editor.input.clone());
    }

    #[test]
    fn confirming_return_input_and_set_editing_to_false() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.append('a'));
        assert!(editor.append('b'));
        assert!(editor.append('c'));
        assert_eq!(String::from("abc"), editor.confirm_input());
        assert!(!editor.editing());
    }
    #[test]
    fn cancelling_set_editing_to_false() {
        let mut editor = Editor::new();
        editor.begin_input(InputKind::Label);
        assert!(editor.append('a'));
        assert!(editor.append('b'));
        assert!(editor.append('c'));
        editor.cancel_input();
        assert!(!editor.editing());
    }
}
