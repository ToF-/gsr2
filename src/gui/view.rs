pub mod main_view;
pub mod palette_area;
pub mod picture_frame;
pub mod treelist_view;

pub struct View {
    pictures_per_row: i32,
    last_pictures_per_row: i32,
    palette_on: bool,
}

impl Default for View {
    fn default() -> Self {
        Self {
           pictures_per_row: 10,
           last_pictures_per_row: 1,
           palette_on: false,
        }
    }
}

impl View {
    pub fn pictures_per_row(&self) -> i32 {
        self.pictures_per_row
    }

    pub fn set_pictures_per_row(&mut self, n: i32) {
        self.last_pictures_per_row = n;
        self.toggle_pictures_per_row();
    }

    pub fn toggle_pictures_per_row(&mut self) {
        std::mem::swap(&mut self.pictures_per_row,&mut self.last_pictures_per_row);
    }
    
    pub fn palette_on(&self) -> bool {
        self.palette_on
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_view_state() {
        let view = View::default();
        assert_eq!(10, view.pictures_per_row());
        assert_eq!(false, view.palette_on());
        
    }

    #[test]
    fn switching_pictures_per_row() {
        let mut view = View::default();
        view.set_pictures_per_row(2);
        assert_eq!(2, view.pictures_per_row());
        view.toggle_pictures_per_row();
        assert_eq!(10, view.pictures_per_row());
        view.toggle_pictures_per_row();
        assert_eq!(2, view.pictures_per_row());
        view.set_pictures_per_row(5);
        assert_eq!(5, view.pictures_per_row());
        view.toggle_pictures_per_row();
        assert_eq!(2, view.pictures_per_row());

    }
}
