//cargo run --example sim1
use asure_pension_core::new::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn name(&mut self) -> String {
        "Sim 00".to_string()
    }
}

fn main() {
    match simulate(Sim::new()) {
        Err(error) => panic!(error),
        _ => ()
    }
}