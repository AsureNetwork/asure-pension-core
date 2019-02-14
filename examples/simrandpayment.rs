use asure_pension_core::*;
use asure_pension_core::types::*;
use asure_pension_core::contributor::*;
use rand::prelude::*;
use rand::thread_rng;

struct Sim {
    rng: ThreadRng,
}

impl Sim {
    pub fn new() -> Self {
        Sim { rng: thread_rng() }
    }
}

impl PensionSimulation for Sim {
    fn new_contributors(&mut self, current_period: Period) -> u64 {
        match current_period {
            1 => 10,
            481 => 10,
            _ => 0,
        }
    }

    fn should_retire(&mut self, contributor: &Contributor, _period: Period) -> bool {
        contributor.contributions.len() == 480
    }

    fn should_contribute(&mut self, _contributor: &Contributor, _period: Period) -> Option<Unit> {
        Some(self.rng.gen_range(0.1, 2.5))
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}