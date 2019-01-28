use asure_pension_core::*;
use asure_pension_core::user::User;
use rand::prelude::*;
use rand::thread_rng;

struct SimulationRandomPayment {
    rng: ThreadRng,

}

impl SimulationRandomPayment {
    pub fn new() -> Self {
        SimulationRandomPayment {
            rng: thread_rng()
        }
    }
}

impl PensionSimulation for SimulationRandomPayment {
    fn name(&mut self) -> String {
        "SimulationRandomPayment".to_string()
    }

    fn create_user(&mut self, current_period: u64) -> u32 {
        match current_period {
            1 => 10,
            481 => 10,
            _ => 0,
        }
    }

    fn should_retire(&mut self, contributor: &User) -> bool {
        contributor.transactions.len() == 480
    }

    fn pay_pension(&mut self, _contributor: &User) -> Option<f64> {
        Some(self.rng.gen_range(0.5, 3.5))
    }
}

fn main() {
    Pension::simulate(SimulationRandomPayment::new());
}