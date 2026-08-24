use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Selection {
    limit: usize,
    range_start: Option<usize>,
    range_end: Option<usize>,
    range_opt: Option<(usize, usize)>,
    selected: HashSet<usize>,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            limit: 0,
            range_start: None,
            range_end: None,
            range_opt: None,
            selected: HashSet::new(),
        }
    }
}

impl Selection {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            range_start: None,
            range_end: None,
            range_opt: None,
            selected: HashSet::new(),
        }
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
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_can_define_a_range() {
        let mut selection = Selection::new(10);
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
        selection.set_range(2);
        assert_eq!(Some(2), selection.range_start());
        selection.set_range(6);
        assert_eq!(Some(6), selection.range_end());
    }

    #[test]
    fn selection_can_define_a_range_backwards() {
        let mut selection = Selection::new(10);
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
        selection.set_range(6);
        selection.set_range(2);
        assert_eq!(Some(2), selection.range_start());
        assert_eq!(Some(6), selection.range_end());
    }
    #[test]
    fn has_a_range_if_range_start_and_range_end_are_set() {
        let mut selection = Selection::new(10);
        assert_eq!(None, selection.range());
        selection.set_range(2);
        assert_eq!(None, selection.range());
        selection.set_range(6);
        assert_eq!(Some((2, 6)), selection.range());
    }
    #[test]
    fn starting_a_new_range_cancels_current_range() {
        let mut selection = Selection::new(10);
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
        selection.set_range(6);
        selection.set_range(2);
        assert_eq!(Some((2, 6)), selection.range());
        selection.set_range(4);
        assert_eq!(None, selection.range());
    }
    #[test]
    fn can_cancel_a_range() {
        let mut selection = Selection::new(10);
        selection.set_range(6);
        selection.set_range(2);
        assert_eq!(Some((2, 6)), selection.range());
        selection.cancel_range();
        assert_eq!(None, selection.range_start());
        assert_eq!(None, selection.range_end());
    }

    #[test]
    fn can_select_and_unselect_an_picture_index() {
        let mut selection = Selection::new(10);
        assert!(!selection.is_selected(0));
        selection.select(9);
        assert!(selection.is_selected(9));
        selection.unselect(9);
        assert!(!selection.is_selected(9));
    }

    #[test]
    fn setting_a_range_selects_included_pictures() {
        let mut selection = Selection::new(10);
        selection.set_range(6);
        assert!(selection.is_selected(6));
        selection.set_range(2);
        selection.select(9);
        assert!(!selection.is_selected(1));
        assert!(selection.is_selected(2));
        assert!(selection.is_selected(3));
        assert!(selection.is_selected(4));
        assert!(selection.is_selected(5));
        assert!(selection.is_selected(6));
        assert!(!selection.is_selected(7));
        assert!(selection.has_selected());
    }
    #[test]
    fn cancelling_a_range_unselects_included_pictures() {
        let mut selection = Selection::new(10);
        selection.set_range(6);
        selection.set_range(2);
        selection.cancel_range();
        assert!(!selection.is_selected(2));
        assert!(!selection.is_selected(3));
        assert!(!selection.is_selected(4));
        assert!(!selection.is_selected(5));
        assert!(!selection.is_selected(6));
    }
    #[test]
    fn unselect_all_cancel_ranges_and_selected() {
        let mut selection = Selection::new(10);
        selection.set_range(6);
        selection.set_range(2);
        selection.select(9);
        selection.unselect_all();
        assert_eq!(None, selection.range());
        assert!(!selection.has_selected());
    }
    #[test]
    fn can_yield_an_ordered_list_of_selected() {
        let mut selection = Selection::new(10);
        selection.set_range(6);
        selection.set_range(2);
        assert_eq!(vec![2, 3, 4, 5, 6], selection.selection());
    }
}
