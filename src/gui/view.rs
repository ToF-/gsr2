pub mod main_view;
pub mod palette_area;
pub mod picture_frame;
pub mod treelist_view;

pub struct View {
    pictures_per_row: i32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            pictures_per_row: 10,
        }
    }
}

impl View {
    pub fn pictures_per_row(&self) -> i32 {
        self.pictures_per_row
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_view_state() {
        let view = View::default();
        assert_eq!(10, view.pictures_per_row());
        
    }
}
