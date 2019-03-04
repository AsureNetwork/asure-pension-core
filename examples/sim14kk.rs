//cargo run --example newsim04
use asure_pension_core::*;
use asure_pension_core::contributor::*;
use asure_pension_core::types::*;

struct Sim;

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }
}

impl PensionSimulation for Sim {
    fn new_contributors(&mut self, current_period: Period) -> u64 {
        match current_period {
            1 => 2,
            480 => 2,
//            960 => 2,
//            1340 => 2,
//            1820 => 2,
            _ => 0,
        }
    }
    fn should_contribute(&mut self, _contributor: &Contributor, period: Period) -> Option<Unit> {
        //inflation
        let period_max = period - 1;//(period-1).min(480);
        let inflation = 5.0;
        let unit = 1.0;
        let year = (period_max / 12) as f64;
        let factor = (inflation + 100.0) / 100.0;
        let result = unit * (factor as f64).powf(year);
        Some(result)
    }
}

fn main() {
    simulate(Sim::new()).unwrap();
}