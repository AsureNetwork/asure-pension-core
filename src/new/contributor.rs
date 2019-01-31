use std::collections::HashMap;
use crate::new::types::*;
use crate::new::user::User;
use crate::new::pensioner::Pensioner;

pub struct Contributor {
    pub contributions: HashMap<Period, Unit>,
    pub dpts: HashMap<Period, Dpt>,
}

impl Contributor {
    pub fn new() -> Self {
        Contributor {
            contributions: HashMap::new(),
            dpts: HashMap::new(),
        }
    }

    pub fn contribute(&mut self, contribution: Unit, period: Period) -> Result<(), String> {
        if contribution <= 0.0 {
            return Err("contribution must be bigger than 0".to_string());
        }
        if period >= 480 {
            return Err("contribution can only be done for 480 periods".to_string());
        }

        match self.contributions.get_mut(&period) {
            Some(contributions) => *contributions += contribution,
            None => {
                self.contributions.insert(period, contribution);
                ()
            }
        }

        Ok(())
    }

    pub fn claim_dpt(&mut self, dpt: Dpt, period: Period) -> Result<(), String> {
        if self.dpts.contains_key(&period) {
            return Err(format!("dpt already claimed for period {}", period));
        }

        self.dpts.insert(period, dpt);
        Ok(())
    }

    pub fn retire(self) -> User {
        User::Pensioner(Pensioner::new(self))
    }

    pub fn contribution_periods(&self) -> u64 {
        self.contributions.len() as u64
    }

    pub fn dpt_total(&self) -> Dpt {
        self.dpts.values().map(|dpt| dpt).sum()
    }
}