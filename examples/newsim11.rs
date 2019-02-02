use asure_pension_core::*;
use asure_pension_core::user::User;

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

    fn create_user(&mut self, current_period: u64) -> u32 {
        if current_period == 1 {
            return 10;
        }
        if current_period > 480 {
            return 0;
        }
        match current_period % (15 + 1) {
            0 => 10,
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
    Pension::simulate(Sim::new());
}