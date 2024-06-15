mod app;
mod config;
mod projects;
mod term;
mod todo;

use app::App;
use term::{init, restore};

fn main() {
    let mut terminal = init().unwrap();
    match App::default().run(&mut terminal) {
        Ok(_) => {}
        Err(_err) => {
            println!("Something went wrong");
        }
    }
    restore().unwrap();
}
