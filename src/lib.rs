#![allow(dead_code)]

use std::cmp::Ordering::Equal;

use crate::common::*;
use crate::user::*;

//use std::cell::RefCell;
//use std::rc::Rc;

pub mod common;
pub mod csvexport;
//pub mod pension;
pub mod transaction;
pub mod user;
pub mod wallet;
pub mod token;
//use std::mem;
//use std::cell::RefCell;
//use std::iter::FromIterator;

pub struct Pension {
    pub total_eth: f64,
    pub total_month_eth: f64,
    pub total_dpt: f64,
    pub total_month_dpt: f64,
    pub total_retirement_dpt: f64,
    pub users: Vec<User>,
    pub current_period: u64,
    pub settings: Settings,
}

struct PensionFold {
    total_eth: f64,
    total_month_eth: f64,
}

impl PensionFold {
    pub fn new() -> PensionFold {
        PensionFold {
            total_eth: 0.0,
            total_month_eth: 0.0,
        }
    }
}

impl Pension {
    pub fn new() -> Pension {
        Pension {
            total_eth: 0.0,
            total_month_eth: 0.0,
            total_dpt: 0.0,
            total_month_dpt: 0.0,
            total_retirement_dpt: 0.0,
            users: Vec::new(),
            current_period: 0,
            settings: Settings::new(),
        }
    }

    pub fn create_users(&mut self, count: u32) {
        for _ in 0..count {
            self.users.push(User::new());
        }
    }

    pub fn start(&mut self) {
        self.current_period += 1;
        self.total_month_eth = 0.0;
        self.settings.current_dpt_bonus = calculations::calculate_dpt_bonus_by_period(self.current_period);
    }

    pub fn add_amount(&mut self, amount: f64) {
        self.total_eth += amount;
        self.total_month_eth += amount;
    }

    pub fn payout(&mut self) {
        let total_retirement_dpt = self.users
            .iter()
            .filter(|u| u.pension_status == PensionStatus::Retirement)
            .fold(0.0, |acc, user| acc + user.wallet.dpt.amount);

        let part = total_retirement_dpt / self.total_dpt;
        let amount = self.total_dpt * part + self.total_month_eth * (1.0 - part);

        self.total_eth -= self.users
            .iter_mut()
            .filter(|u| u.pension_status == PensionStatus::Retirement)
            .fold(0.0, |total_eth, user| {
//            if user.pension_received_months < user.pension_receive_months {
//                user.pension_received_months += 1;
//            } else {
//                if user.pension_received_months <= user.pension_receive_months {
//                    user.activated_dpt = 0.0;
//                    user.pension_status = PensionStatus::Done
//                }
//                return total_eth;
//            }

                let my_dpt = user.wallet.dpt.amount;
                let my_part = my_dpt / total_retirement_dpt;

                user.wallet.pension_eth += my_part * amount;
                return total_eth + my_part * amount;
            });
    }

    pub fn end(&mut self) {
        let period = self.current_period;

        let period_amounts = self.users
            .iter()
            .flat_map(|user| &user.transactions)
            .filter(|tx| tx.period == period)
            .map(|tx| tx.amount)
            .collect::<Vec<_>>();

        if period_amounts.len() == 0 {
            return;
        }

        self.settings.current_contribution_value = calculations::calculate_contribution_value(
            self.settings.current_contribution_value,
            self.settings.current_contribution_value_degree,
            &period_amounts,
        );

        let mut sorted_period_amounts: Vec<f64> = period_amounts.to_vec();
        sorted_period_amounts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

        let max = *sorted_period_amounts.last().unwrap();

        self.total_month_dpt = 0.0;

        for user in &mut self.users {
            if let Some(tx) = user.transactions.iter().find(|tx| tx.period == period) {
                let dpt = calculations::calculate_dpt(
                    tx.amount,
                    self.settings.current_contribution_value,
                    self.settings.current_dpt_bonus,
                    max,
                );

                self.total_month_dpt += dpt;
                self.total_dpt += dpt;
                user.wallet.dpt.amount += dpt;
                user.last_dpt = dpt;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_users() {
        let mut pension = Pension::new();
        pension.create_users(5);

        assert_eq!(pension.users.len(), 5);
    }

//    #[test]
//    fn start_should_create_a_new_period() {
//        let mut pension = Pension::new();
//        //assert!(pension.current_period.is_none());
//        pension.start();
//        assert!(pension.current_period);
//    }

//    #[test]
//    fn calculate_avg_points_should_be_zero_to_zero_five() {
//        let mut pension = Pension::new();
//
//        pension.current_period = 1;
//        let result_zero_five = pension.calculate_dpt_bonus();
//        println!("{}", result_zero_five);
//        assert_eq!(result_zero_five, 0.5f64);
//
//        pension.current_period = 40 * 12;
//        let zero = pension.calculate_dpt_bonus();
//        println!("{}", zero);
//        assert_eq!(zero, 0.0f64);
//    }
}
