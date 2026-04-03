pub type Cover = Option<usize>;

pub fn bool_to_cover(value: bool) -> Cover {
    match value {
        false => None,
        true => Some(0),
    }
}

pub fn cover_to_bool(cover: Cover) -> bool {
    cover.is_some()
}

pub fn cover_sort_key(cover: Cover) -> usize {
    usize::MAX - cover.unwrap_or_default()
}
