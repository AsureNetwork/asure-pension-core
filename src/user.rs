use std::sync::atomic::{self, AtomicUsize};
use crate::transaction::*;
use crate::wallet::*;

static USER_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;

pub struct User {
    pub id: usize,
    pub name: String,
    pub wallet: Wallet,
    pub pension_status: PensionStatus,
    pub pension_payment_months: u128,
    pub pension_receive_months: u128,
    pub pension_received_months: u128,
    pub activated_dpt: f64,
    pub last_dpt: f64,
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
            pension_status: PensionStatus::Run,
            pension_payment_months: 0,
            pension_receive_months: 0,
            pension_received_months: 0,
            activated_dpt: 0.0,
            last_dpt: 0.0,
            transactions: Vec::new(),
        }
    }

    pub fn get_pension_receive_months(&self) -> u128 {
        (self.pension_payment_months * self.pension_payment_months) / 480
    }

    pub fn activate_retirement(&mut self) -> bool {
        self.pension_status = PensionStatus::Retirement;
        true
    }

    pub fn pay(&mut self, period: u64, amount: f64) -> Result<(), String> {
        if self.pension_status == PensionStatus::Retirement {
            return Err(String::from("Already retired"));
        }

        if self.pension_payment_months >= 480 {
            return Err(format!("Already payed {} month", self.pension_payment_months));
        }

        let tx = Transaction::new(period, amount);
        self.wallet.eth -= tx.amount;
        self.pension_payment_months += 1;
        self.transactions.push(tx);

        Ok(())
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

        assert!(user.pension_status == PensionStatus::Run);
        user.activate_retirement();
        assert!(user.pension_status == PensionStatus::Retirement);
    }


    #[test]
    fn pay_should_throw_error_already_retired(){
        let mut user:User = User :: new();
        user.pension_status = PensionStatus::Retirement;
        assert!(user.pay(12, 100.0).is_err(), "Already retired")
    }

    #[test]
    fn pay_should_throw_error_already_payed(){
        let mut user:User = User::new();
        user.pension_payment_months = 480;
        assert!(user.pay(12, 100.0).is_err(), "Already payed {} month", user.pension_payment_months);
    }

    #[test]
    fn pay_create_single_transaction(){
        let mut user:User = User::new();
        user.pension_payment_months = 0;
        user.wallet.eth = 100.0;

        user.pay(12, 10.0);

        assert_eq!(user.transactions.len(), 1);
        assert_eq!(user.wallet.eth, 90.0);
        assert_eq!(user.pension_payment_months, 1);
    }

    #[test]
    fn pay_ten_years_period(){
        let mut user:User = User::new();
        user.pension_payment_months = 0;
        user.wallet.eth = 1000.0;

        for period in 0..480 {
            user.pay(period, 1.0);
        }

        assert_eq!(user.transactions.len(), 480);
        assert_eq!(user.wallet.eth, 520.0);
        assert_eq!(user.pension_payment_months, 480);

    }

}


