//cargo run --example sim1
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
        "Sim 05".to_string()
    }

    fn create_user(&mut self, current_period: u64) -> u32 {
        match current_period {
            1 => 10,
            _ => 0,
        }
    }

    fn should_retire(&mut self, contributor: &User) -> bool {
        match contributor.id {
            0 => contributor.transactions.len() == 48,
            1 => contributor.transactions.len() == 48 * 2,
            2 => contributor.transactions.len() == 48 * 3,
            3 => contributor.transactions.len() == 48 * 4,
            4 => contributor.transactions.len() == 48 * 5,
            5 => contributor.transactions.len() == 48 * 6,
            6 => contributor.transactions.len() == 48 * 7,
            7 => contributor.transactions.len() == 48 * 8,
            8 => contributor.transactions.len() == 48 * 9,
            9 => contributor.transactions.len() == 480,
            _ => false,
        }
    }

    fn pay_pension(&mut self, _contributor: &User) -> Option<f64> {
        Some(1.0)
    }
}

fn main() {
    Pension::simulate(Sim::new());
}