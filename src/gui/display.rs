use itertools::Itertools;
use std::collections::HashSet;
use crate::model::rank::Rank;
use crate::env::default_values::{EXPAND_ON_SYMBOL, FULL_SIZE_ON_SYMBOL};
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
pub fn picture_label_display(label: &str, rank: Rank, with_focus: Option<char>) -> String {
    format!(
        "{} {} {}",
        if let Some(symbol) = with_focus {
            symbol
        } else {
            ' '
        },
        label,
        rank,
    )
}

fn tag_display(tags: Tags) -> String {
    match tags.len() {
        0 => String::from(""),
        _ => format!("| {} |", tags.iter().join(" ")),
    }
}

pub fn title_display(controller: &Controller) -> String {
    format!(
        "#{} {} {} {} {} {} {} {}{}",
        controller.navigator().position(),
        page_display(controller),
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
    )
}
