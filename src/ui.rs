use cursive;
use cursive::views::{SelectView, LinearLayout, TextView};
use cursive::traits::*;
use cursive::align::HAlign;
use cursive::{Printer, XY};
use cursive::vec::Vec2;

use textwrap::fill;
use serde_json;

use api::{LatestTopic, Topic, Category, Post};
use util::*;

pub fn new_topic_selector(topics: Vec<LatestTopic>, width: usize, categories: &Vec<Category>) -> SelectView<LatestTopic> {
    let mut tv = SelectView::new();
    for topic in topics {
        tv.add_item(format!("{} {} {} {} {}",
            pad_or_trunc(topic.title.clone(), width-50), 
            pad_or_trunc(lookup_category(categories, topic.category_id).unwrap(), 15), 
            pad_or_trunc(format_int(topic.posts_count), 5), 
            pad_or_trunc(format_int(topic.views), 6), 
            pad_or_trunc(datestring_to_humanstring(&topic.last_posted_at), 15))
                    , topic);
    }
    tv
}

pub fn new_multipost_view(posts: Vec<Post>) -> LinearLayout {
    let mut lin = LinearLayout::vertical();
    for post in posts {
        lin.add_child(PostView::new(post))
    }
    lin
} 

pub struct PostView {
    post: Post,
    human_date: String,
    size: Vec2,
    text_rows: Vec<String>,
}

impl PostView {
    pub fn new(in_post: Post) -> PostView {
        PostView { 
            size: Vec2::new(0,0),
            human_date: datestring_to_humanstring(&in_post.created_at),
            text_rows: Vec::new(),
            post: in_post,
        }
    }

    fn compute_rows(&mut self, size: Vec2) {
        self.text_rows = fill(&self.post.raw, size.x-2).lines().map(ToOwned::to_owned).collect();
    }
}

impl cursive::view::View for PostView {
    fn layout(&mut self, size: Vec2) {
        self.compute_rows(size);
        self.size = size; 
    }

    fn draw(&self, printer: &Printer) {
        printer.print_box((0,0), self.size, false);
        printer.print((1,1), &self.post.username);
        printer.print((self.size.x-self.human_date.len()-1, 1), &self.human_date);
        for (i, line) in self.text_rows.iter().enumerate() {
            printer.print((1,3+i), line)
        }
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.compute_rows(Vec2::new(constraint.x-10, constraint.y));
        self.size = Vec2::new(constraint.x-10, self.text_rows.len()+4);
        self.size
    }

}
