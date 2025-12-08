pub type Label = String;

const LAST_ASCII: &str = "~";

pub fn from(s: &str) -> Label {
    s.to_string()
}

pub fn sort_key(s: &str) -> String {
    if !s.is_empty() {
        s.to_string()
    } else {
        LAST_ASCII.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_sort_key_order_empty_labels_as_last() {
        assert_eq!("ALPHA".to_string(), sort_key("ALPHA"));
        assert_eq!("~".to_string(), sort_key(""));
    }
}
