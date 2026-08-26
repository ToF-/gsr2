use std::collections::HashSet;
use std::mem;

#[derive(Debug, Clone)]
pub struct Selection {
    range_start: Option<usize>,
    range_end: Option<usize>,
    selected: HashSet<usize>,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            range_start: None,
            range_end: None,
            selected: HashSet::new(),
        }
    }
}

impl Selection {
    pub fn count(&self) -> usize {
        self.selected.len()
    }
    pub fn range_start(&self) -> Option<usize> {
        self.range_start
    }

    pub fn range_end(&self) -> Option<usize> {
        self.range_end
    }

    pub fn range(&self) -> Option<(usize, usize)> {
        if let Some(start) = self.range_start()
            && let Some(end) = self.range_end()
        {
            Some((start, end))
        } else {
            None
        }
    }

    pub fn has_selected(&self) -> bool {
        !self.selected.is_empty()
    }

    pub fn indices(&mut self) -> Vec<usize> {
        let mut result: Vec<usize> = self.selected.clone().into_iter().collect();
        result.sort();
        result
    }

    pub fn contains(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    pub fn select(&mut self, index: usize) {
        self.selected.insert(index);
    }

    pub fn unselect(&mut self, index: usize) {
        let _ = self.selected.remove(&index);
        if self.selected.is_empty() {
            self.cancel_range();
        }
    }

    pub fn unselect_all(&mut self) {
        self.cancel_range();
    }

    pub fn cancel_range(&mut self) {
        self.selected.clear();
        self.range_start = None;
        self.range_end = None
    }

    fn update_selected(&mut self) {
        if let Some((start, end)) = self.range() {
            self.selected.clear();
            for index in start..=end {
                self.select(index)
            }
        }
    }
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.range_start = Some(start);
        self.range_end = Some(end);
        self.update_selected();
    }
    pub fn set_range_end(&mut self, index: usize) {
        if self.range().is_some() {
            self.cancel_range()
        }
        if self.range_start.is_none() {
            self.range_start = Some(index);
            self.select(index);
        } else {
            self.range_end = Some(index);
            if self.range_end < self.range_start {
                mem::swap(&mut self.range_start, &mut self.range_end)
            }
        };
        self.update_selected();
    }
    pub fn positions(&mut self) -> Vec<usize> {
        let mut result: Vec<usize> = self.selected.clone().into_iter().collect();
        result.sort();
        result
    }
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_can_define_a_range() {
        let mut selection = Selection::default();
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
        selection.set_range_end(2);
        assert_eq!(Some(2), selection.range_start());
        selection.set_range_end(6);
        assert_eq!(Some(6), selection.range_end());
    }

    #[test]
    fn selection_can_define_a_range_backwards() {
        let mut selection = Selection::default();
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
        selection.set_range_end(6);
        selection.set_range_end(2);
        assert_eq!(Some(2), selection.range_start());
        assert_eq!(Some(6), selection.range_end());
    }
    #[test]
    fn has_a_range_if_range_start_and_range_end_are_set() {
        let mut selection = Selection::default();
        assert_eq!(None, selection.range());
        selection.set_range_end(2);
        assert_eq!(None, selection.range());
        selection.set_range_end(6);
        assert_eq!(Some((2, 6)), selection.range());
    }
    #[test]
    fn starting_a_new_range_cancels_current_range() {
        let mut selection = Selection::default();
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
        selection.set_range_end(6);
        selection.set_range_end(2);
        assert_eq!(Some((2, 6)), selection.range());
        selection.set_range_end(4);
        assert_eq!(None, selection.range());
    }
    #[test]
    fn can_cancel_a_range() {
        let mut selection = Selection::default();
        selection.set_range_end(6);
        selection.set_range_end(2);
        assert_eq!(Some((2, 6)), selection.range());
        selection.cancel_range();
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
    }

    #[test]
    fn can_select_and_unselect_an_picture_index() {
        let mut selection = Selection::default();
        assert!(!selection.contains(0));
        selection.select(9);
        assert!(selection.contains(9));
        selection.unselect(9);
        assert!(!selection.contains(9));
    }

    #[test]
    fn setting_a_range_selects_included_pictures() {
        let mut selection = Selection::default();
        selection.set_range_end(6);
        assert!(selection.contains(6));
        selection.set_range_end(2);
        selection.select(9);
        assert!(!selection.contains(1));
        assert!(selection.contains(2));
        assert!(selection.contains(3));
        assert!(selection.contains(4));
        assert!(selection.contains(5));
        assert!(selection.contains(6));
        assert!(!selection.contains(7));
        assert!(selection.has_selected());
    }
    #[test]
    fn cancelling_a_range_unselects_included_pictures() {
        let mut selection = Selection::default();
        selection.set_range_end(6);
        selection.set_range_end(2);
        selection.cancel_range();
        assert!(!selection.contains(2));
        assert!(!selection.contains(3));
        assert!(!selection.contains(4));
        assert!(!selection.contains(5));
        assert!(!selection.contains(6));
    }
    #[test]
    fn unselect_all_cancel_ranges_and_selected() {
        let mut selection = Selection::default();
        selection.set_range_end(6);
        selection.set_range_end(2);
        selection.select(9);
        selection.unselect_all();
        assert_eq!(None, selection.range());
        assert!(!selection.has_selected());
    }
    #[test]
    fn can_yield_an_ordered_list_of_selected() {
        let mut selection = Selection::default();
        selection.set_range_end(6);
        selection.set_range_end(2);
        assert_eq!(vec![2, 3, 4, 5, 6], selection.indices());
    }
}
