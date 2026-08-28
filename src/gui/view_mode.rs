#[derive(PartialEq, Clone, Debug)]
pub enum ViewMode {
    Normal,
    Expanded,
    FullSize,
}

impl ViewMode {
    pub fn next(&self) -> Self {
        match self {
            ViewMode::Normal => ViewMode::Expanded,
            ViewMode::Expanded => ViewMode::FullSize,
            ViewMode::FullSize => ViewMode::Normal,
        }
    }
}
