//cargo run --example sim2
//#![feature(exclusive_range_pattern)]

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
        "Sim 04".to_string()
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
        match _contributor.pension_payment_months {
            0...59 => Some(1.0),
            60...239 => Some(0.5),
            240...359 => Some(0.25),
            360...479 => Some(0.1),
            _ => Some(0.0),
        }
    }
}

fn main() {
    Pension::simulate(Sim::new());
}