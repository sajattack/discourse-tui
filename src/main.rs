#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate hyper;

extern crate app_dirs;
extern crate cursive;
extern crate openssl;
extern crate reqwest;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate chrono;
extern crate chrono_humanize;
extern crate textwrap;
extern crate percent_encoding;

mod api;
mod ui;
mod util;

use cursive::Cursive;
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::views::*;
use cursive::traits::*;
use cursive::align::HAlign;
use cursive::event::Key;

use app_dirs::{AppDataType, AppInfo, app_dir};

use std::env;
use std::fs;
use std::fs::File;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::rc::Rc;
use std::path::Path;

use api::PartialApi;
use api::Api;
use api::{Category, Post, LatestTopic};

const APP_INFO: AppInfo = AppInfo {
    name: "discourse-tui",
    author: "Paul Sajna",
};

fn main() {
    let config_dir =  app_dir(AppDataType::UserConfig, &APP_INFO, "").unwrap();
    let tmp_dir = &env::temp_dir().join("discourse-tui");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let reader = File::open(config_dir.as_path().join("config.json")).unwrap();
            let api: Api = serde_json::from_reader(reader).unwrap();
            run_with_api(api);
        },
        2 => {
            if args[1].contains("discourse://auth_redirect") {
                let reader = File::open(tmp_dir.join("partial-api.json")).unwrap();
                let pa: PartialApi = serde_json::from_reader(reader).unwrap();
                let payload = args[1].replace("discourse://auth_redirect?payload=", "");
                let api = pa.decrypt_key(payload).unwrap();
                let writer = File::create(config_dir.as_path().join("config.json")).unwrap();
                serde_json::to_writer_pretty(writer, &api);
            } else if args[1].starts_with("http") {
                let api = Api::new_unauthenticated(&args[1]);
                run_with_api(api);
            }
        },
        3 => {
             if args[1].contains("--new-key") {
                let pa = PartialApi::gen_key_url(args[2].clone()).unwrap();
                if !tmp_dir.exists() {
                    fs::create_dir(tmp_dir);
                }
                let writer = File::create(tmp_dir.join("partial-api.json")).unwrap();
                serde_json::to_writer(writer, &pa);
                println!("{}", &pa.api_authorize_url);
            }
        }
        _ => {},
    }
}

fn run_with_api(api: Api) {
    let api = Arc::new(api);
    let config_dir =  app_dir(AppDataType::UserConfig, &APP_INFO, "").unwrap();
    let mut siv = Cursive::ncurses();
    siv.load_theme_file(config_dir.as_path().join("theme.toml"));
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(LinearLayout::vertical()
        .child(TextView::new(
                StyledString::styled("", Effect::Bold))
                .h_align(HAlign::Center)
                .with_id("forum_name"))
        .child(DummyView.fixed_height(1))
        .with_id("main_layout")
        );
    siv.set_fps(10);
    let cb_sink = mpsc::Sender::clone(&siv.cb_sink());
    let api_copy = Arc::clone(&api);
    let io_thread = thread::spawn(move || {
        let api = api_copy;
        let categories = api.get_categories().unwrap();
        let latest_topics = api.get_latest_topics().unwrap();
        let forum_name = api.get_forum_name().unwrap();
        cb_sink.send(Box::new(move |s: &mut Cursive| {
            let mut forum_name_view: ViewRef<TextView> = s.find_id("forum_name").unwrap();
            forum_name_view.set_content(forum_name);
            let mut main_layout: ViewRef<LinearLayout> = s.find_id("main_layout").unwrap();
            let width = s.screen_size().x;
            let mut topic_selector = ui::new_topic_selector(latest_topics, width, &categories);
            let api_copy = Arc::clone(&api);
            topic_selector.set_on_submit(move |s, lt| {
                let api = Arc::clone(&api_copy);
                let topic = api.get_topic_by_id(lt.id).unwrap();
                let posts: Vec<Post>;
                if topic.posts_count > 10 {
                    posts = api.get_posts_in_topic(&topic, topic.posts_count-25, 25).unwrap();
                } else {
                    posts = api.get_posts_in_topic(&topic, 0, topic.posts_count).unwrap()
                }
                let mut topic_view = OnEventView::new(LinearLayout::vertical());
                topic_view.get_inner_mut().add_child(TextView::new(topic.title.clone()));
                topic_view.get_inner_mut().add_child(ui::new_multipost_view(posts));
                let api_copy = Arc::clone(&api);
                if api.has_key() {
                    topic_view.set_on_event('r', move |s| {
                        let api = Arc::clone(&api_copy);
                        let topic_id = topic.id;
                        s.screen_mut().add_layer(Dialog::around(TextArea::new().with_id("text_area"))
                            .button("Reply", move |s_| {
                                let text_area: ViewRef<TextArea> = s_.find_id("text_area").unwrap();
                                api.make_post_in_topic(topic_id, text_area.get_content().to_string());
                                s_.pop_layer();
                            })
                            .dismiss_button("Cancel"));
                    });
                }
                let main_screen = s.active_screen();
                s.add_active_screen();
                s.screen_mut().add_layer(topic_view);
                s.add_global_callback(Key::Esc, move |s___| s___.set_screen(main_screen));
            });
            main_layout.add_child(topic_selector);
        }));
    });
    siv.run();
}
