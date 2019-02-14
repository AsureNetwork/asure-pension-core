//cargo run --example newsim05
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
    fn new_contributors(&mut self, current_period: Period) -> u64 {
        match current_period {
            1 => 2,
            24 => 2,
            48 => 2,
            72 => 2,
            96 => 2,
            _ => 0,
        }
    }
    fn should_retire(&mut self, contributor: &Contributor, _period: Period) -> bool {
        match contributor.id() {
            0 => contributor.contributions.len() == 48,
            1 => contributor.contributions.len() == 48 * 2,
            2 => contributor.contributions.len() == 48 * 3,
            3 => contributor.contributions.len() == 48 * 4,
            4 => contributor.contributions.len() == 48 * 5,
            5 => contributor.contributions.len() == 48 * 6,
            6 => contributor.contributions.len() == 48 * 7,
            7 => contributor.contributions.len() == 48 * 8,
            8 => contributor.contributions.len() == 48 * 9,
            9 => contributor.contributions.len() == 480,
            _ => false,
        }
    }
    fn should_contribute(&mut self, _contributor: &Contributor, _period: Period) -> Option<Unit> {
        Some(1.0)
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}
