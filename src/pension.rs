//use std::mem;
//use std::cell::RefCell;

use crate::period::*;
use crate::user::*;
use crate::transaction::*;

pub struct Pension {
    pub period: Vec<Period>,
    pub total_eth: u64,
    pub total_month_eth: u64,
    pub total_dpt: u64,
    pub total_month_dpt: u64,
    pub total_retirement_dpt: u64,
    pub users: Vec<User>,
    pub current_period: Period,
    pub current_period2: Option<Period>,
}

struct PensionFold {
    txs: Vec<Transaction>,
    total_eth: u64,
    total_month_eth: u64,
}

impl PensionFold {
    pub fn new() -> PensionFold {
        PensionFold {
            txs: vec![],
            total_eth: 0,
            total_month_eth: 0,
        }
    }
}

impl Pension {
    pub fn new() -> Pension {
        Pension {
            period: Vec::new(),
            total_eth: 0,
            total_month_eth: 0,
            total_dpt: 0,
            total_month_dpt: 0,
            total_retirement_dpt: 0,
            users: Vec::new(),
            current_period: Period::new(),
            current_period2: Option::None,
        }
    }

    pub fn create_users(&mut self, count: u32) {
        for _ in 0..count {
            self.users.push(User::new());
        }
    }

    pub fn start(&mut self) {
        self.current_period = Period::new();
        self.current_period2 = Option::Some(Period::new());
    }

    pub fn pay(&mut self) {
        self.total_month_eth = 0;

        let mut result = self.users
            .iter_mut()
            .filter(|u| u.pension_status == PensionStatus::Run)
            .fold(PensionFold::new(), |mut state, user| {
                if user.pension_payment_months == 480 {
                    user.activate_retirement();
                    return state;
                }

                let mut tx = Transaction::new();
                // tx.user = user;
                tx.amount = 20;

                user.wallet.eth -= tx.amount;
                user.pension_payment_months += 1;

                state.total_eth += tx.amount;
                state.total_month_eth += tx.amount;
                state.txs.push(tx);

                return state;
            });

        self.current_period.txs.append(&mut result.txs);
        self.total_eth = result.total_eth;
        self.total_month_eth = result.total_month_eth;
    }

    pub fn payout(&self) {}

    pub fn calculate_points(&self) -> u128 {
        return 0;
    }

    pub fn end(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::pension::*;

    #[test]
    fn create_users() {
        let mut pension = Pension::new();
        pension.create_users(5);

        assert_eq!(pension.users.len(), 5);
    }

    #[test]
    fn start_should_create_a_new_period() {
        let mut pension = Pension::new();
        assert!(pension.current_period.is_none());

        pension.start();
        assert!(pension.current_period.is_some());
    }
}
