use cursive::views::{ListView, LinearLayout, TextView};
use cursive::traits::*;
use api::Topic;
use api::Category;
use util::datestring_to_humanstring;
use util::format_int;

pub fn listview_from_topics(topics: Vec<Topic>, width: usize, categories: &Vec<Category>) -> ListView {
    let mut lv = ListView::new();
    for topic in topics.iter() {
        lv.add_child("", LinearLayout::horizontal()
            .child(TextView::new(topic.title.clone()).fixed_width(width-45))
            .child(TextView::new(lookup_category(categories, topic.category_id).unwrap()).fixed_width(15))
            .child(TextView::new(format_int(topic.reply_count)).fixed_width(5))
            .child(TextView::new(format_int(topic.views)).fixed_width(6))
            .child(TextView::new(datestring_to_humanstring(&topic.last_posted_at)).fixed_width(15)));
    }
    lv
}

pub fn lookup_category(categories: &Vec<Category>, id: i32) -> Option<String> {
    for category in categories.iter() {
        if category.id == id {
            return Some(category.name.clone())
        }
    }
    None
}
