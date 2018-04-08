#[macro_use]
extern crate serde_derive;

extern crate app_dirs;
extern crate tempdir;
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

mod api;
mod ui;
mod util;

use cursive::Cursive;
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::views::*;
use cursive::traits::*;
use cursive::align::HAlign;

use app_dirs::{AppDataType, AppInfo, app_dir};
use tempdir::TempDir;

use std::env;
use std::fs::File;
use std::thread;
use std::sync::mpsc;

use api::PartialApi;
use api::Api;
use api::{Category, Post, LatestTopic};

const APP_INFO: AppInfo = AppInfo {
    name: "discourse-tui",
    author: "Paul Sajna",
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_dir =  app_dir(AppDataType::UserConfig, &APP_INFO, "").unwrap();
    let tmp_dir = TempDir::new("discourse-tui").unwrap();
    match args.len() {
        1 => {
            let mut siv = Cursive::new();
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
            let io_thread = thread::spawn(move || {
                let reader = File::open(config_dir.as_path().join("config.json")).unwrap();
                let api: Api = serde_json::from_reader(reader).unwrap();
                let categories = api.get_categories().unwrap();
                let latest_topics = api.get_latest_topics().unwrap();
                let forum_name = api.get_forum_name().unwrap();
                cb_sink.send(Box::new(move |s: &mut Cursive| {
                    let mut forum_name_view: ViewRef<TextView> = s.find_id("forum_name").unwrap();
                    forum_name_view.set_content(forum_name);
                    let mut main_layout: ViewRef<LinearLayout> = s.find_id("main_layout").unwrap();
                    let width = s.screen_size().x;
                    let mut topic_selector = ui::new_topic_selector(latest_topics, width, &categories);
                    topic_selector.set_on_submit(move |s, lt| {    
                        let topic = api.get_topic_by_id(lt.id).unwrap();
                        let posts: Vec<Post>;
                        if topic.posts_count > 3 {
                            posts = api.get_posts_in_topic(&topic, topic.posts_count-3, 3).unwrap();
                        } else {
                            posts = api.get_posts_in_topic(&topic, 0, topic.posts_count).unwrap()
                        }
                        s.add_active_screen();
                        s.screen_mut().add_layer(
                            ui::new_multipost_view(posts))
                    });
                    main_layout.add_child(topic_selector);

                    
                }));
            });
            siv.run();
        },
        2 => {
            if args[1].contains("discourse://auth_redirect") {
                let reader = File::open(tmp_dir.path().join("partial-api.json")).unwrap();
                let pa: PartialApi = serde_json::from_reader(reader).unwrap();
                let payload = args[1].replace("discourse://auth_redirect?payload=", "");
                let api = pa.decrypt_key(payload).unwrap();
                let writer = File::create(config_dir.as_path().join("config.json")).unwrap();
                serde_json::to_writer_pretty(writer, &api);
            } else if args[1].contains("--new-key") {
                let pa = PartialApi::gen_key_url("https://community.frontrowcrew.com"
                                                 .to_string()).unwrap();
                let writer = File::create(tmp_dir.path().join("partial-api.json")).unwrap();
                serde_json::to_writer(writer, &pa);
                println!("{}", &pa.api_authorize_url);
            }
        },
        _ => {},
    }
}
