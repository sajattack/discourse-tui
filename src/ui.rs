use cursive::views::{SelectView, LinearLayout, TextView};
use cursive::traits::*;
use api::Topic;
use api::Category;
use util::datestring_to_humanstring;
use util::format_int;

pub fn init_topicview(topics: Vec<Topic>, width: usize, categories: &Vec<Category>) -> SelectView {
    let mut tv = SelectView::new();
    for topic in topics.iter() {
        tv.add_item_str(format!("{:w1$} {:w2$} {:w3$} {:w4$} {:w5$}",
            topic.title.clone(), 
            lookup_category(categories, topic.category_id).unwrap(), 
            format_int(topic.reply_count), 
            format_int(topic.views), 
            datestring_to_humanstring(&topic.last_posted_at), 
            w1=width-45,w2=15,w3=5,w4=6,w5=15))
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
