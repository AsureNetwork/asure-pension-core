use asure_pension_core::*;
use asure_pension_core::contributor::Contributor;
use asure_pension_core::types::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn should_contribute(&mut self, contributor: &Contributor, _period: Period) -> Option<Unit> {
        match contributor.id() {
            0 => Some(0.1),
            _ => Some(1.0),
        }
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}