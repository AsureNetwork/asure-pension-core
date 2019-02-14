//cargo run --example newsim05
use asure_pension_core::*;
use asure_pension_core::contributor::*;
use asure_pension_core::types::*;
use asure_pension_core::pensioner::Pensioner;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn new_contributors(&mut self, current_period: Period) -> u64 {
        match current_period {
            1 => 5,
            480 => 5,
            _ => 0,
        }
    }

    fn should_claim_pension(&mut self, pensioner: &Pensioner, period: Period) -> bool {
        match pensioner.contributor.id() {
            0...5 => period > 480,
            _ => period > 960
        }
    }

    fn should_contribute(&mut self, contributor: &Contributor, _period: Period) -> Option<Unit> {
        match contributor.id() {
            6 => Some(10000.0),
            _ => Some(1.0),
        }
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}
