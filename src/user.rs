//use chrono::{DateTime, Utc};

pub struct User {
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
}

#[derive(PartialEq)]
pub enum PensionStatus {
    Run,
    Retirement,
    Done,
}

impl User {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> User {
        User {
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
        }
    }

    pub fn get_pension_receive_months(&self) -> u128 {
        (self.pension_payment_months * self.pension_payment_months) / 480
    }

    pub fn activate_retirement(&mut self) -> bool {
        self.retirement = true;
        true
    }
}


pub struct Wallet {
    pub dpt: Token,
    pub eth: f64,
    pub pension_eth: f64,
    pub tokens: Vec<Token>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            dpt: Token { name: String::from("DTP"), amount: 0.0 },
            eth: 0.0,
            pension_eth: 0.0,
            tokens: Vec::new(),
        }
    }
}

pub struct Token {
    pub name: String,
    pub amount: f64
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


