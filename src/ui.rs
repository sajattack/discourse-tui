use cursive::views::{ListView, LinearLayout, TextView};
use cursive::traits::*;
use api::Topic;
use util::datestring_to_humanstring;

pub fn listview_from_topics(topics: Vec<Topic>, width: usize) -> ListView {
    let mut lv = ListView::new();
    for topic in topics.iter() {
        lv.add_child("", LinearLayout::horizontal()
            .child(TextView::new(topic.title.clone()).fixed_width(width-38))
            .child(TextView::new(topic.category_id.to_string()).fixed_width(5))
            .child(TextView::new(topic.reply_count.to_string()).fixed_width(5))
            .child(TextView::new(topic.views.to_string()).fixed_width(5))
            .child(TextView::new(datestring_to_humanstring(&topic.last_posted_at)).fixed_width(15)));
    }
    lv
}


