//cargo run --example newsim05
use asure_pension_core::*;
//use asure_pension_core::contributor::*;
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
            1 => 1,
            960 => 10,
            _ => 0,
        }
    }

    fn should_claim_pension(&mut self, pensioner: &Pensioner, period: Period) -> bool {
        match pensioner.contributor.id() {
            0 => period <= 940 || period >= 2470,
            _ => period >= 2000
        }
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}
