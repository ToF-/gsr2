use crate::model::selection::Selection;
use itertools::Itertools;
use crate::model::rank::Rank;
use crate::env::default_values::{COVER_SYMBOL, EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL, ORDER_SYMBOL};
use crate::model::order::Order;
use crate::gui::controller::Controller;
use crate::model::image_data::Tags;

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
pub fn picture_label_display(label: &str, rank: Rank, cover: bool, with_focus: Option<char>) -> String {
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

fn cover_display(cover: bool) -> String {
    if cover {
        COVER_SYMBOL.to_string()
    } else {
        "".to_string()
    }
}

fn display_selection(selection: &Selection) -> String {
    if !selection.is_empty() {
        format!("=[{}]", selection.tags().into_iter().join("|"))
    } else {
        "".to_string()
    }
}
fn tag_display(tags: Tags) -> String {
    match tags.len() {
        0 => String::from(""),
        _ => {
            let mut labels: Vec<String>  = tags.into_iter().collect();
            labels.sort();
            format!("| {} |", labels.iter().join(" "))
        },
    }
}

pub fn title_display(controller: &Controller) -> String {
    format!(
        "{} #{} {} {} {} {} {} {} {} {}{} {}",
        cover_display(
            match controller.current_picture().image_data() {
                Some(image_data) => image_data.cover,
                None => false,
            }),
        controller.navigator().position(),
        page_display(controller),
        order_display(controller.gallery().order()),
        controller.current_picture().file_name(),
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
