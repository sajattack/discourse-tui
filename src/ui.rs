use cursive::views::{SelectView};
use cursive::traits::*;
use api::Topic;
use api::Category;
use util::datestring_to_humanstring;
use util::format_int;
use util::pad_or_trunc;

pub fn init_topicview(topics: Vec<Topic>, width: usize, categories: &Vec<Category>) -> SelectView {
    let mut tv = SelectView::new();
    for topic in topics.iter() {
        tv.add_item_str(format!("{} {} {} {} {}",
            pad_or_trunc(topic.title.clone(), width-50), 
            pad_or_trunc(lookup_category(categories, topic.category_id).unwrap(), 15), 
            pad_or_trunc(format_int(topic.reply_count), 5), 
            pad_or_trunc(format_int(topic.views), 6), 
            pad_or_trunc(datestring_to_humanstring(&topic.last_posted_at), 15)));
    }
    tv
}

pub fn lookup_category(categories: &Vec<Category>, id: i32) -> Option<String> {
    for category in categories.iter() {
        if category.id == id {
            return Some(category.name.clone())
        }
    }
    None
}
