//cargo run --example sim2
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
        "Sim 03".to_string()
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
        match _contributor.id {
            0 => Some(0.1),
            _ => Some(1.0),
        }
    }
}

fn main() {
    Pension::simulate(Sim::new());
}