use asure_pension_core::*;
use asure_pension_core::contributor::*;
use asure_pension_core::types::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn new_contributors(&mut self, period: Period) -> u64 {
        if period == 1 {
            return 4;
        }
        if period >= 480 {
            return 0;
        }
        match period % 24 {
            0 => 10,
            _ => 0,
        }
    }

    fn should_retire(&mut self, contributor: &Contributor, _period: Period) -> bool {
        if (contributor.id() % 5 == 0 && contributor.contributions.len() == 240)
            ||
            (contributor.id() % 10 == 0 && contributor.contributions.len() == 360)
            ||
            contributor.contributions.len() == 480
            { return true; }
        return false;
    }

    fn should_contribute(&mut self, _contributor: &Contributor, _period: Period) -> Option<Unit> {
        Some(1.0)
    }

    fn should_print(&mut self, period: Period) -> bool {
        period == 1415
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}