use asure_pension_core::*;
use asure_pension_core::user::User;

struct Simulation0;

impl Simulation0 {
    pub fn new() -> Self {
        Simulation0 {}
    }
}

impl PensionSimulation for Simulation0 {
    fn name(&mut self) -> String {
        "Simulation0".to_string()
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
        Some(1.0)
    }
}

fn main() {
    Pension::simulate(Simulation0::new());
}