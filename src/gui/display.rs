use crate::env::default_values::{
    COVER_SYMBOL, EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL, ORDER_SYMBOL, PICTURE_SIZE_THRESHOLD,
    SMALL_PICTURE_SYMBOL,
};
use crate::file::paths::file_name_from;
use crate::gui::view_mode::ViewMode;
use crate::gui::view_state::ViewState;
use crate::model::cover::Cover;
use crate::model::image_data::FileSize;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use itertools::Itertools;

fn select_pattern_display(view_state: &ViewState) -> String {
    match &view_state.saved_locations.last() {
        Some(location) => match location.predicate() {
            Some(predicate) => format!("[{}]", predicate.to_string()),
            None => "".to_string(),
        },
        None => "".to_string(),
    }
}

fn find_pattern_display(view_state: &ViewState) -> String {
    match &view_state.finder {
        Some(finder) => match finder.predicate() {
            Some(predicate) => format!("[{}]", predicate.to_string()),
            None => "".to_string(),
        },
        None => "".to_string(),
    }
}

fn view_mode_display(view_state: &ViewState) -> String {
    if view_state.settings.single_view() {
        match view_state.settings.single_view_mode() {
            ViewMode::Normal => String::from(""),
            ViewMode::Expanded => String::from(EXPAND_ON_SYMBOL),
            ViewMode::FullSize => String::from(FULL_SIZE_ON_SYMBOL),
        }
    } else {
        String::from("")
    }
}

fn covers_only_display(view_state: &ViewState) -> String {
    if view_state.settings.covers_only() {
        String::from(COVER_SYMBOL)
    } else {
        String::from("")
    }
}
fn page_display(view_state: &ViewState, position: usize) -> String {
    let page = if view_state.settings.single_view() {
        "".to_string()
    } else {
        let len = view_state.gallery.len();
        let pictures_per_row = view_state.settings.pictures_per_row() as usize;
        let page_size = pictures_per_row * pictures_per_row;
        let number = 1 + position / page_size;
        let total = if len <= page_size {
            1
        } else {
            1 + (len / page_size)
        };
        format!("{number}/{total}")
    };
    if view_state.settings.single_view() {
        String::from("")
    } else {
        format!(
            "p{}/{}",
            view_state.navigator.current_page(),
            view_state.navigator.total_pages(),
        )
    };
    page
}

fn name_display(view_state: &ViewState, picture: &Picture) -> String {
    if view_state.settings.file_path_on() {
        picture.file_path()
    } else {
        picture.file_name()
    }
}

fn order_display(view_state: &ViewState) -> String {
    format!("{}{}", ORDER_SYMBOL, view_state.gallery.order())
}
pub fn picture_label_display(
    label: &str,
    rank: Rank,
    cover: Cover,
    with_focus: Option<char>,
    size_opt: Option<FileSize>,
) -> String {
    format!(
        "{}{} {} {} {}",
        small_picture_display(size_opt),
        cover_display(cover),
        with_focus.unwrap_or(' '),
        label,
        rank,
    )
}

fn directory_display(view_state: &ViewState) -> String {
    if let Some(folder) = view_state.gallery.sub_folder() {
        file_name_from(&folder)
    } else {
        String::from("")
    }
}

fn cover_display(cover: Cover) -> String {
    match cover {
        None | Some(0) => "".to_string(),
        Some(count) => format!("{}({})", COVER_SYMBOL, count),
    }
}

fn label_display(view_state: &ViewState) -> String {
    let label = view_state.gallery.current_picture().label();
    if !label.is_empty() {
        format!("<{}>", label)
    } else {
        String::from("")
    }
}

fn date_display(view_state: &ViewState) -> String {
    if view_state.settings.file_date_on() {
        view_state.gallery.current_picture().modified_time_display()
    } else {
        String::from("")
    }
}

fn size_display(view_state: &ViewState) -> String {
    if view_state.settings.file_size_on() {
        view_state.gallery.current_picture().file_size_display()
    } else {
        String::from("")
    }
}
fn category_display(picture: &Picture) -> String {
    match picture.image_data() {
        None => String::from(""),
        Some(data) => match data.category_name() {
            None => String::from(""),
            Some(name) => format!("#{}", name.to_uppercase()),
        },
    }
}

fn tag_display(picture: &Picture) -> String {
    match picture.image_data() {
        None => String::from(""),
        Some(data) => match data.tags().len() {
            0 => String::from(""),
            _ => {
                let mut labels: Vec<String> = data.tags().into_iter().collect();
                labels.sort();
                format!("| {} |", labels.iter().join(" "))
            }
        },
    }
}

pub fn small_picture_display(size_opt: Option<FileSize>) -> String {
    (if let Some(size) = size_opt {
        if size < PICTURE_SIZE_THRESHOLD {
            SMALL_PICTURE_SYMBOL
        } else {
            " "
        }
    } else {
        "?"
    })
    .to_string()
}

fn selected_count_display(view_state: &ViewState) -> String {
    let sel_count = if view_state.selection.is_empty() {
        "".to_string()
    } else {
        let count = view_state.selection.count();
        format!("[{count}]")
    };
    sel_count
}

pub fn title_display(view_state: &ViewState) -> String {
    let select = select_pattern_display(view_state);
    let pattern = find_pattern_display(view_state);
    let covers_only = covers_only_display(view_state);
    let picture = view_state.gallery.current_picture();
    view_state.gallery.current_picture().file_name();
    let folder = directory_display(view_state);
    let position = view_state.gallery.current_picture_index();
    let page = page_display(view_state, position);
    let sel_count = selected_count_display(view_state);
    let order = order_display(view_state);
    let small = small_picture_display(picture.image_data().map(|data| data.size()));
    let cover = cover_display(picture.cover());
    let name = name_display(view_state, &picture);
    let label = label_display(view_state);
    let rank = picture.rank().to_string();
    let category = category_display(&picture);
    let tags = tag_display(&picture);
    let date = date_display(view_state);
    let size = size_display(view_state);
    let view = view_mode_display(view_state);
    format!(
        "{select}{pattern}{covers_only} {folder} #{position} {page} {sel_count} {order}                     {cover} {name} {label} {rank} {category} {tags} {date} {size} {view}{small} "
    )
}
