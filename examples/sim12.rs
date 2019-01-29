#![feature(exclusive_range_pattern)]

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
        "Sim 12".to_string()
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

    fn pay_pension(&mut self, contributor: &User) -> Option<f64> {
        match contributor.id {
            0..10 => Some(1.0),
            10..20 => Some(0.5),
            _ => Some(0.0),
        }
    }
}

fn main() {
    Pension::simulate(Sim::new());
}