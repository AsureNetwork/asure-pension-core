use std::sync::atomic::{self, AtomicUsize};
use std::collections::HashMap;
use crate::new::types::*;
use crate::new::user::User;
use crate::new::pensioner::Pensioner;
use crate::calculations::*;

static USER_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;

pub struct Contributor {
    id: usize,
    wallet: Unit,
    pub contributions: HashMap<Period, Unit>,
    pub dpts: HashMap<Period, Dpt>,
}

impl Contributor {
    pub fn new() -> Self {
        Contributor {
            id: USER_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            wallet: 10000000.0,
            contributions: HashMap::new(),
            dpts: HashMap::new(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }



    pub fn wallet(&self) -> Unit {
        self.wallet
    }

    pub fn contribute(&mut self, contribution: Unit, period: Period) -> Result<(), String> {
        if contribution <= 0.0 {
            return Err("contribution must be bigger than 0".to_string());
        }
        if contribution > self.wallet {
            return Err("contribution to bigger than wallet".to_string());
        }
        if self.contributions.len() > 480 {
            return Err("contribution can only be done for 480 periods".to_string());
        }

        match self.contributions.get_mut(&period) {
            Some(contributions) => *contributions += contribution,
            None => {
                self.contributions.insert(period, contribution);
                ()
            }
        }

        self.wallet -= contribution;

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

    pub fn has_retire_months(&self) -> bool {
        self.allowed_pension_periods() >= 1
    }

    pub fn allowed_pension_periods(&self) -> u64 {
        let periods = self.contribution_periods();
        calculate_entitlement_months(periods)
    }

    pub fn dpt_total(&self) -> Dpt {
        self.dpts.values().map(|dpt| dpt).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::new::contributor::Contributor;

    #[test]
    fn contribute_contribution_must_be_bigger_than_0() {
        let mut contributor = Contributor::new();
        assert!(contributor.contribute(0.0,1).is_err());
        assert!(contributor.contribute(-1.0, 1).is_err())
    }

    #[test]
    fn contribute_contribution_bigger_than_wallet() {
        let mut contributor = Contributor::new();
        assert!(contributor.contribute(10000001.0,1).is_err());
    }

    #[test]
    fn contribute_max_period() {
        let mut contributor = Contributor::new();
        for periond in 1..481  {
            contributor.contribute(1.0, periond).unwrap();
        }
        assert_eq!(contributor.contributions.len(), 480);
        contributor.contribute(1.0, 481).unwrap();
        assert!(contributor.contribute(1.0,482).is_err());
    }

    #[test]
    fn contribute_contribution_done() {
        let mut contributor = Contributor::new();

        for periond in 1..481  {
            contributor.contribute(1.0, periond).unwrap();
        }
        assert_eq!(contributor.wallet(), 9999520.0);
    }

    #[test]
    fn claim_dpt_dpt_already_claimed_for_period(){
        let mut contributor = Contributor::new();
        contributor.dpts.insert(1, 1.0);
        assert!(contributor.claim_dpt(1.0, 1).is_err());
    }

    #[test]
    fn claim_dpt_done(){
        let mut contributor = Contributor::new();
        contributor.dpts.insert(1, 1.0);
        assert_eq!(contributor.dpts.len(), 1);
    }

    #[test]
    fn dpt_total(){
        let mut contributor = Contributor::new();
        for periond in 1..481  {
            contributor.dpts.insert(periond, 1.0);
        }
        assert_eq!(contributor.dpt_total(), 480.0);
    }
}