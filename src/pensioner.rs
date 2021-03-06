use std::collections::HashMap;
use crate::contributor::Contributor;
use crate::types::*;
use crate::user::User;
use crate::doneuser::DoneUser;

pub struct Pensioner {
    pub contributor: Contributor,
    pensions: HashMap<Period, Unit>,
}

impl Pensioner {
    pub fn new(contributor: Contributor) -> Self {
        Pensioner {
            contributor,
            pensions: HashMap::new(),
        }
    }

    pub fn claim_pension(&mut self, period: Period, rate: f64) -> Unit {
        let pension = self.contributor.dpt_total() * rate;

        let total_pension = match self.pensions.get(&period) {
            Some(current_pension) => current_pension + pension,
            None => pension
        };
        self.pensions.insert(period, total_pension);

        total_pension
    }

    pub fn try_finish(self) -> User {
        if self.pension_periods() >= self.contributor.allowed_pension_periods() {
            User::Done(DoneUser::new(self))
        } else {
            User::Pensioner(self)
        }
    }

    pub fn pension_periods(&self) -> u64 {
        self.pensions.len() as u64
    }

    pub fn total_pension(&self) -> Unit {
        self.pensions.values().sum()
    }
}