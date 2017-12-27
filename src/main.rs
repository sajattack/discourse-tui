extern crate cursive;

use cursive::Cursive;

fn main() {
    let mut siv = Cursive::new();
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}

