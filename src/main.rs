mod app;
mod config;
mod projects;
mod term;
mod todo;

use app::App;
use term::{init, restore};

fn main() {
    let mut terminal = init().unwrap();
    let _app = App::default().run(&mut terminal).unwrap();
    restore().unwrap();
}
