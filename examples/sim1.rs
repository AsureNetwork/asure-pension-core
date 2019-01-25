//cargo run --example sim1
use asure_pension_core::*;
use asure_pension_core::user::User;

struct Simulation1;

impl Simulation1 {
    pub fn new() -> Self {
        Simulation1 {}
    }
}

impl PensionSimulation for Simulation1 {
    fn name(&mut self) -> String {
        "Simulation1".to_string()
    }

    fn create_user(&mut self, current_period: u64) -> u32 {
        match current_period {
            1 => 10,
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
    Pension::simulate(Simulation1::new());
}