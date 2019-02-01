//cargo run --example sim00
use asure_pension_core::new::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
}

fn main() {
    simulate(Sim::new()).unwrap();
}