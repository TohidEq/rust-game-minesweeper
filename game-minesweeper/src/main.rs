use playground::Playground;

pub mod colors;
pub mod config;
pub mod playground;

fn main() {
    println!("Hello, world!");
    let mut playground = Playground::new(4, 4);
    playground.create_playground();
}
