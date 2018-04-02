extern crate cursive;
extern crate openssl;
extern crate reqwest;

mod api;

use cursive::Cursive;
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::views::*;
use cursive::traits::*;
use cursive::align::HAlign;

fn main() {
    
    let mut siv = Cursive::new();
    siv.load_theme_file("/home/paul/.config/discourse-tui/theme.toml");
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(LinearLayout::vertical()
        .child(TextView::new(StyledString::styled("Front Row Crew Forum", Effect::Bold)).h_align(HAlign::Center))
        .child(DummyView.fixed_height(1))
        .child(ListView::new()
                .child("", LinearLayout::horizontal()
                    .child(TextView::new("The Best Game You Can Name (Ice Hockey)")
                            .fixed_width(50))
                    .child(TextView::new("Sports").fixed_width(15))
                    .child(TextView::new("146").fixed_width(5))
                    .child(TextView::new("2.9K").fixed_width(5))
                    .child(TextView::new("23m").fixed_width(5)))
                .child("", LinearLayout::horizontal()
                       .child(TextView::new("Professional Wrestling").fixed_width(50))
                       .child(TextView::new("Sports").fixed_width(15))
                       .child(TextView::new("247").fixed_width(5))
                       .child(TextView::new("4.2k").fixed_width(5))
                       .child(TextView::new("2h").fixed_width(5)))
                .child("", LinearLayout::horizontal()
                       .child(TextView::new("Media Analysis and Criticism")
                            .fixed_width(50))
                       .child(TextView::new("General").fixed_width(15))
                       .child(TextView::new("49").fixed_width(5))
                       .child(TextView::new("1.9k").fixed_width(5))
                       .child(TextView::new("3h").fixed_width(5)))
                .child("", LinearLayout::horizontal()
                       .child(TextView::new("Utena").fixed_width(50))
                       .child(TextView::new("Animation").fixed_width(15))
                       .child(TextView::new("0").fixed_width(5))
                       .child(TextView::new("11").fixed_width(5))
                       .child(TextView::new("4h").fixed_width(5))))


        .fixed_width(90));
    siv.run();
    
   // println!("{}", api::gen_key("https://community.frontrowcrew.com".to_string()).unwrap());
}

