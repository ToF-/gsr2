use crate::env::default_values::{
    COVER_SYMBOL, EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL, ORDER_SYMBOL,
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
) -> String {
    format!(
        "{} {} {} {}",
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

pub fn title_display(controller: &Controller) -> String {
    if controller.state().display_information_on() {
        format!("{}", controller.current_picture().file_path())
    } else {
        format!(
            "{} #{} {} {} {} {} {} {} {} {} {}{} {}",
            cover_display(controller.current_picture().cover()),
            controller.navigator().position(),
            page_display(controller),
            order_display(controller.gallery().order()),
            controller.current_picture().file_name(),
            label_display(controller.current_picture().label()),
            match controller.current_picture().image_data() {
                Some(image_data) => image_data.rank(),
                None => Rank::NoStar,
            },
            match controller.current_picture().image_data() {
                Some(image_data) => tag_display(image_data.tags),
                None => "".to_string(),
            },
            if controller.state().display_date_on() {
                controller.current_picture().modified_time_display()
            } else {
                String::from("")
            },
            if controller.state().display_size_on() {
                controller.current_picture().file_size_display()
            } else {
                String::from("")
            },
            expand_display(controller.state().expand_on()),
            full_size_display(controller.state().full_size_on()),
            display_selection(&controller.gallery().selection()),
        )
    }
}
