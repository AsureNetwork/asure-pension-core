use asure_pension_core::*;
use asure_pension_core::types::*;
use asure_pension_core::contributor::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn name(&mut self) -> String {
        "Sim 12".to_string()
    }

    fn new_contributors(&mut self, period: Period) -> u64 {
        match period {
            1 => 10,
            481 => 10,
            _ => 0,
        }
    }

    fn should_retire(&mut self, contributor: &Contributor, _period: Period) -> bool {
        contributor.contributions.len() == 480
    }

    fn should_contribute(&mut self, contributor: &Contributor, _period: Period) -> Option<Unit> {
        match contributor.id() {
            0...9 => Some(1.0),
            10...20 => Some(0.5),
            _ => Some(0.0),
        }
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}