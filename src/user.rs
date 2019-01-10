use std::sync::atomic::{self, AtomicUsize};
use crate::transaction::*;
use crate::wallet::*;

static USER_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;

pub struct User {
    pub id: usize,
    pub name: String,
    pub wallet: Wallet,
    pub retirement: bool,
    pub pension_status: PensionStatus,
    pub pension_payment_months: u128,
    pub pension_receive_months: u128,
    pub pension_received_months: u128,
    pub eth: u128,
    pub pension_eth: u128,
    pub total: u128,
    pub dpt: u128,
    pub activated_dpt: f64,
    pub transactions: Vec<Transaction>,
}

#[derive(PartialEq)]
pub enum PensionStatus {
    Run,
    Retirement,
    Done,
}

static mut ID_GENERATOR: u64 = 0;

impl User {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Self {
        User {
            id: USER_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            name: String::from("UserName"),
            wallet: Wallet::new(),
            retirement: false,
            pension_status: PensionStatus::Run,
            pension_payment_months: 0,
            pension_receive_months: 0,
            pension_received_months: 0,
            eth: 0,
            pension_eth: 0,
            total: 0,
            dpt: 0,
            activated_dpt: 0.0,
            transactions: Vec::new(),
        }
    }

    pub fn get_pension_receive_months(&self) -> u128 {
        (self.pension_payment_months * self.pension_payment_months) / 480
    }

    pub fn activate_retirement(&mut self) -> bool {
        self.retirement = true;
        true
    }

    pub fn pay_into_pension(&mut self, period: u64, amount: f64) {
        let tx = Transaction::new(period, amount);

        self.wallet.eth -= tx.amount;
        self.pension_payment_months += 1;
        self.transactions.push(tx);
    }
}


#[cfg(test)]
mod tests {
    use crate::user::*;

    #[test]
    fn get_pension_receive_months_for_ten_years() {
        let mut user = User::new();
        user.pension_payment_months = 120;

        assert_eq!(user.get_pension_receive_months(), 30);
    }

    #[test]
    fn should_activate_retirement() {
        let mut user = User::new();

        assert_eq!(user.retirement, false);
        user.activate_retirement();
        assert_eq!(user.retirement, true);
    }
}


