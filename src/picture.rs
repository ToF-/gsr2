pub struct Picture {
    file_name: String,
}

impl Picture {
    pub fn new(file_name: &str) -> Self {
        Picture {
            file_name: file_name.to_string(),
        }
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_picture_as_file_name_which_is_the_full_path_and_file_name_on_the_file_system() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colors.png"),
            picture.file_name()
        )
    }
}
