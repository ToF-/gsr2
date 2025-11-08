use crate::model::image_data::FileSize;
use crate::env::default_values::{
    SMALL_PICTURE_SYMBOL, COVER_SYMBOL, EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL, ORDER_SYMBOL, PICTURE_SIZE_THRESHOLD
};
use crate::gui::controller::Controller;
use crate::model::cover::Cover;
use crate::model::label::Label;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::selection::Selection;
use crate::model::tags::Tags;
use itertools::Itertools;

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

fn page_display(controller: &Controller) -> String {
    if controller.state().single_view() {
        String::from("")
    } else {
        format!(
            "p{}/{}",
            controller.navigator().current_page(),
            controller.navigator().total_pages(),
        )
    }
}

fn order_display(order: Order) -> String {
    format!("{}{}", ORDER_SYMBOL, order)
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
        if let Some(symbol) = with_focus {
            symbol
        } else {
            ' '
        },
        label,
        rank,
    )
}

fn cover_display(cover: Cover) -> String {
    match cover {
        None | Some(0) => "".to_string(),
        Some(count) => format!("{} {} ", COVER_SYMBOL.to_string(), count),
    }
}

fn display_selection(selection: &Selection) -> String {
    if !selection.is_empty() {
        format!("=[{}]", selection.tags().into_iter().join("|"))
    } else {
        "".to_string()
    }
}

fn label_display(label: Label) -> String {
    if label.len() > 0 {
        format!("<{}>", label)
    } else {
        String::from("")
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
    format!("{}",
        if let Some(size) = size_opt {
            if size < PICTURE_SIZE_THRESHOLD {
                SMALL_PICTURE_SYMBOL
            } else {
                " "
            }
        } else {
            "?"
        })
}

pub fn title_display(controller: &Controller) -> String {
    if controller.state().display_information_on() {
        format!("{}", controller.current_picture().file_path())
    } else {
        let order: Order;
        let current_picture = controller.current_picture();
        let selection: Selection = Selection::from_args(&controller.args());

        if let Ok(gallery) = controller.repository().gallery_rc().try_borrow() {
            order = gallery.order();
        } else {
            panic!("can't borrow")
        };
        format!(
            "{}{} #{} {} {} {} {} {} {} {} {} {}{} {}",
            small_picture_display(current_picture.image_data().map(|d| d.size())),
            cover_display(current_picture.cover()),
            controller.navigator().position(),
            page_display(controller),
            order_display(order),
            current_picture.file_name(),
            label_display(current_picture.label()),
            match current_picture.image_data() {
                Some(image_data) => image_data.rank(),
                None => Rank::NoStar,
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
            display_selection(&selection),
            )
    }
}
