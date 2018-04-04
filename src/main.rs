#[macro_use]
extern crate serde_derive;

extern crate cursive;
extern crate openssl;
extern crate reqwest;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate chrono;
extern crate chrono_humanize;

mod api;
mod ui;
mod util;

use cursive::Cursive;
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::views::*;
use cursive::traits::*;
use cursive::align::HAlign;

use std::env;
use std::fs::File;
use std::thread;
use std::sync::mpsc;

use api::PartialApi;
use api::Api;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let mut siv = Cursive::new();
            siv.load_theme_file("/home/paul/.config/discourse-tui/theme.toml");
            siv.add_global_callback('q', |s| s.quit());
            siv.add_fullscreen_layer(LinearLayout::vertical()
                .child(TextView::new(
                        StyledString::styled("Front Row Crew Forum", Effect::Bold))
                        .h_align(HAlign::Center))
                .child(DummyView.fixed_height(1))
                .with_id("main_layout")
                );
            siv.set_fps(10);
            let cb_sink = mpsc::Sender::clone(&siv.cb_sink());
            let io_thread = thread::spawn(move || {
                let reader = File::open("/home/paul/.config/discourse-tui/config.json").unwrap();
                let api: Api = serde_json::from_reader(reader).unwrap();
                match api.get_latest_topics() {
                    Err(err) => println!("{}", err),
                    Ok(topics) => {
                        cb_sink.send(Box::new(|s: &mut Cursive| {
                            let mut main_layout: ViewRef<LinearLayout> = s.find_id("main_layout").unwrap();
                            main_layout.add_child(ui::listview_from_topics(topics, s.screen_size().x));
                        }));
                    }
                }     
            });
            siv.run();
            
        },
        2 => {
            if args[1].contains("discourse://auth_redirect") {
                let reader = File::open("/tmp/discourse-tui/partial-api.json").unwrap();
                let pa: PartialApi = serde_json::from_reader(reader).unwrap();
                let payload = args[1].replace("discourse://auth_redirect?payload=", "");
                let api = pa.decrypt_key(payload).unwrap();
                let writer = File::create("/home/paul/.config/discourse-tui/config.json").unwrap();
                serde_json::to_writer_pretty(writer, &api);
            } else if args[1].contains("--new-key") {
                let pa = PartialApi::gen_key_url("https://community.frontrowcrew.com"
                                                 .to_string()).unwrap();
                let writer = File::create("/tmp/discourse-tui/partial-api.json").unwrap();
                serde_json::to_writer(writer, &pa);
                println!("{}", &pa.api_authorize_url);
            }
        },
        _ => {},
    }
}

/*
 *fn show_listview(siv: &mut Cursive) {
 *    let reader = File::open("/home/paul/.config/discourse-tui/config.json").unwrap();
 *    //let api: Api = serde_json::from_reader(reader).unwrap();
 *    let api = Api {
 *        base_url: "https://discourse.redox-os.org".to_string(),
 *        client_id: "".to_string(),
 *        api_key: "".to_string(),
 *    };
 *    match api.get_latest_topics() {
 *    Err(err) => println!("{}", err),
 *        Ok(topics) => {
 *            siv.add_layer(ui::listview_from_topics(topics));
 *        },
 *    }
 *}
 */
