//cargo run --example sim1
use asure_pension_core::core::{run};
use asure_pension_core::common::Settings;

fn main() {
    println!("Hello, world!");
    run();
    let mut settings = Settings::new();
    settings.eth=10;
}