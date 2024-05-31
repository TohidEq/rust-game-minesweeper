use std::io::stdout;

use crossterm::{terminal, ExecutableCommand};
use playground::Playground;

pub mod colors;
pub mod config;
pub mod playground;

fn main() {
    let mut sc = stdout();
    println!("Hello, world!");
    let mut playground = Playground::new(50, 10);
    playground.create_playground();
    // clear screen
    sc.execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    playground.draw_playground(&mut sc);
}
