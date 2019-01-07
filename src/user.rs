use std::collections::linked_list::LinkedList;

use chrono::{DateTime, Utc};

pub struct User {
    pub name: String,
    pub wallet: Wallet,
    pub pension_status: u128,
    pub pension_payment_months: u128,
    pub pension_recived_months: u128,
    pub eth: u128,
    pub pension_eth: u128,
    pub total: u128,
    pub dpt: u128,
    pub activated_dtp: u128,
}

impl User {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> User {
        User {
            name: String::new(),
            wallet: Wallet::new(),
            pension_status: 0,
            pension_payment_months: 0,
            pension_recived_months: 0,
            eth: 0,
            pension_eth: 0,
            total: 0,
            dpt: 0,
            activated_dtp: 0,
        }
    }

    pub fn get_pension_recieve_months(&self) -> u128 {
        (&self.pension_payment_months * &self.pension_payment_months) / 480
    }
}


pub struct Wallet {
    pub eth: i64,
    pub pension_eth: i64,
    pub tokens: LinkedList<Token>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            eth: 0,
            pension_eth: 0,
            tokens: LinkedList::new(),
        }
    }
}

pub struct Token {
    pub amount: i64,
    pub created: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use crate::user::*;

        #[test]
        fn get_pension_recieve_months_for_ten_years() {
            let mut user = User::new();
            user.pension_payment_months = 120;
            assert_eq!(user.get_pension_recieve_months(), 30);
        }
    }
}


