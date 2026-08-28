use crate::model::picture::Picture;
use crate::env::default_values::{
    COVER_SYMBOL, EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL, ORDER_SYMBOL, PICTURE_SIZE_THRESHOLD,
    SMALL_PICTURE_SYMBOL,
};
use crate::gui::view_state::ViewState;
use crate::model::cover::Cover;
use crate::model::image_data::FileSize;
use crate::model::label::Label;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::selection_criteria::SelectionCriteria;
use crate::model::tags::Tags;
use itertools::Itertools;
use std::cmp::max;


fn expand_display(on: bool) -> String {
    match on {
        false => String::from(""),
        true => String::from(EXPAND_ON_SYMBOL),
    }
}

fn full_size_display(on: bool) -> String {
    match on {
        false => String::from(""),
        true => String::from(FULL_SIZE_ON_SYMBOL),
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
    todo!()
}

fn cover_display(cover: Cover) -> String {
    match cover {
        None | Some(0) => "".to_string(),
        Some(count) => format!("{}({})", COVER_SYMBOL, count),
    }
}

fn display_selection(selection: &SelectionCriteria) -> String {
    if !selection.is_empty() {
        format!("=[{}]", selection.tags().into_iter().join("|"))
    } else {
        "".to_string()
    }
}

fn label_display(label: Label) -> String {
    if !label.is_empty() {
        format!("<{}>", label)
    } else {
        String::from("")
    }
}

fn category_display(picture: &Picture) -> String {
    match picture.image_data() {
        None => String::from(""),
        Some(data) => match data.category_name() {
            None => String::from(""),
            Some(name) => format!("#{name}"),
        },
    }
}

fn tag_display(tags: Tags) -> String {
    match tags.len() {
        0 => String::from(""),
        _ => {
            let mut labels: Vec<String> = tags.into_iter().collect();
            labels.sort();
            format!("| {} |", labels.iter().join(" "))
        }
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
    let picture = view_state.gallery.current_picture();
    view_state.gallery.current_picture().file_name();
    let folder = "".to_string();
    let position = view_state.gallery.current_picture_index();
    let page = page_display(view_state, position);
    let sel_count = selected_count_display(view_state);
    let order = order_display(view_state);
    let small = small_picture_display(picture.image_data().map(|data| data.size()));
    let cover = cover_display(picture.cover());
    let name = name_display(view_state, &picture);
    let label = {
        let label = view_state.gallery.current_picture().label();
        if !label.is_empty() {
            format!("<{}>", label)
        } else {
            String::from("")
        }
    };
    let rank = picture.rank().to_string();
    let category = category_display(&picture);
    let tags = "".to_string();
    let date = "".to_string();
    let size = "".to_string();
    let expand = "".to_string();
    let full = "".to_string();
    let selection = "".to_string();

    format!(
        "{folder} #{position} {page} {sel_count} {order}                     {cover} {name} {label} {rank} {category} {tags} {date} {size} {expand}{full} {selection} {small} "
    )
}

/*
if controller.state().display_path_on() {
    controller.current_picture().file_path().to_string()
} else {
    let order: Order;
    let current_picture = controller.current_picture();
    let selection_criteria: SelectionCriteria =
        SelectionCriteria::from_args(&controller.command_line_arguments());

    if let Ok(gallery) = controller.repository().gallery_rc().try_borrow() {
        order = gallery.order();
    } else {
        panic!("can't borrow")
    };
    format!(
        "{}{}{} #{} {} {} {} {} {} {} {} {} {} {} {}{} {}",
        directory_display(controller),
        small_picture_display(current_picture.image_data().map(|d| d.size())),
        cover_display(current_picture.cover()),
        controller.navigator().position(),
        page_display(controller),
        selected_count_display(controller),
        order_display(order),
        current_picture.file_name(),
        label_display(current_picture.label()),
        match current_picture.image_data() {
            Some(image_data) => image_data.rank(),
            None => Rank::NoStar,
        },
        match current_picture.image_data() {
            Some(image_data) => category_display(image_data.category_name()),
            None => "".to_string(),
        },
        match current_picture.image_data() {
            Some(image_data) => tag_display(image_data.tags),
            None => "".to_string(),
        },
        if controller.state().display_date_on() {
            current_picture.modified_time_display()
        } else {
            String::from("")
        },
        if controller.state().display_size_on() {
            current_picture.file_size_display()
        } else {
            String::from("")
        },
        expand_display(controller.state().expand_on()),
        full_size_display(controller.state().full_size_on()),
        display_selection(&selection_criteria),
    )
}*/
