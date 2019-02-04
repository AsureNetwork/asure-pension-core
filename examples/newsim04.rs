//cargo run --example newsim04
use asure_pension_core::new::*;
use asure_pension_core::new::types::*;
use asure_pension_core::new::contributor::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn new_contributors(&mut self, current_period: Period) -> u64 {
        match current_period {
            1 => 10,
            _ => 0,
        }
    }
    fn should_contribute(&mut self, contributor: &Contributor, _period: Period) -> Option<Unit> {
        match contributor.contributions.len() {
            0...59 => Some(1.0),
            60...239 => Some(0.5),
            240...359 => Some(0.25),
            360...479 => Some(0.1),
            _ => Some(0.0),
        }
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}