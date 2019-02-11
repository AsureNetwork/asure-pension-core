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
    fn name(&mut self) -> String {
        "Sim 11".to_string()
    }

    fn new_contributors(&mut self, current_period: Period) -> u64 {
        if current_period > 1340 {
            return 0;
        }
        if (current_period == 1)
            ||
            (current_period % 12 == 0) {
            return 1;
        }
        return 0;
    }

    fn should_retire(&mut self, contributor: &Contributor, _period: Period) -> bool {
        contributor.contributions.len() == 480
    }

    fn should_contribute(&mut self, _contributor: &Contributor, _period: Period) -> Option<Unit> {
        Some(1.0)
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}