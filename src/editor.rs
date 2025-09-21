#[derive(Debug)]
pub struct Editor {}

impl Editor {
    pub fn new() -> Editor {
        Editor {}
    }

    pub fn editing(&self) -> bool {
        false
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

}
