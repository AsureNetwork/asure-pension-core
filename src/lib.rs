#![allow(dead_code)]

use std::cmp::Ordering::Equal;

use crate::common::*;
use crate::user::*;

pub mod common;
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
    }

    pub fn add_amount(&mut self, amount: f64) {
        self.total_eth += amount;
        self.total_month_eth += amount;
    }

//    pub fn pay(&mut self) {
//        self.total_month_eth = 0.0;
//
//        let period = self.current_period;
//
//        let result = self.users
//            .iter_mut()
//            .filter(|u| u.pension_status == PensionStatus::Run)
//            .fold(PensionFold::new(), |mut state, user| {
//                if user.pension_payment_months == 480 {
//                    user.activate_retirement();
//                    return state;
//                }
//
//                let amount = 20.0;
//                user.pay_into_pension(period, amount);
//
//                state.total_eth += amount;
//                state.total_month_eth += amount;
//
//                return state;
//            });
//
//        self.total_eth = result.total_eth;
//        self.total_month_eth = result.total_month_eth;
//    }

    pub fn payout(&mut self) {
        let mut users = self.users
            .iter_mut()
            .filter(|u| u.pension_status == PensionStatus::Retirement);

        let total_retirement_dpt = users.by_ref().fold(0.0, |acc, user| acc + user.activated_dpt);
        let part = total_retirement_dpt / self.total_dpt;
        let amount = self.total_dpt * part + self.total_month_eth * (1.0 - part);

        self.total_eth -= users.by_ref().fold(0.0, |total_eth, user| {
            if user.pension_received_months < user.pension_receive_months {
                user.pension_received_months += 1;
            } else {
                if user.pension_received_months <= user.pension_receive_months {
                    user.activated_dpt = 0.0;
                    user.pension_status = PensionStatus::Done
                }
                return total_eth;
            }

            let my_dpt = user.activated_dpt;
            let my_part = my_dpt / total_retirement_dpt;

            user.wallet.pension_eth += my_part * amount;
            return total_eth + my_part * amount;
        });
    }

    pub fn calculate_points(&self, amount: f64, min: f64, max: f64) -> f64 {
        let result = calculations::calculate_points(
            self.settings.current_contribution_value,
            self.settings.current_avg_points,
            amount,
            min,
            max,
        );
        result
    }

    pub fn end(&mut self) {
        let period = self.current_period;

        let period_amounts = self.users
            .iter()
            .flat_map(|user| &user.transactions)
            .filter(|tx| tx.period == period)
            .map(|tx| tx.amount);

        if period_amounts.clone().count() == 0 {
            return;
        }

        let plus = period_amounts.clone().filter(|amount| *amount > self.settings.current_contribution_value).count();
        let minus = period_amounts.clone().filter(|amount| *amount < self.settings.current_contribution_value).count();

        self.settings.current_contribution_value = if plus > minus {
            self.settings.current_contribution_value * (1.0 + self.settings.current_points_degree / 100.0)
        } else {
            self.settings.current_contribution_value * (1.0 - self.settings.current_points_degree / 100.0)
        };

//        let sum: f64 = period_amounts.clone().sum();
//        let avg: f64 = sum / period_amounts.clone().count() as f64;

        let mut sorted_period_amounts: Vec<f64> = period_amounts.clone().collect();
        sorted_period_amounts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

        let min = *sorted_period_amounts.first().unwrap();
        let max = *sorted_period_amounts.last().unwrap();

        self.total_month_dpt = 0.0;

        for user in &mut self.users {
            if let Some(tx) = user.transactions.iter().find(|tx| tx.period == period) {
//                let amount = (*self).calculate_points(tx.amount, min, max);
                let amount = calculations::calculate_points(
                    self.settings.current_contribution_value,
                    self.settings.current_avg_points,
                    tx.amount,
                    min,
                    max,
                );
                self.settings.tokens += amount;
                user.wallet.dpt.amount += amount;
                self.total_month_dpt += amount;
            }
        }
    }

    pub fn calculate_avg_points(&self) -> f64 {
        assert_ne!(self.current_period, 0);
        if self.current_period >= 40 * 12 {
            return 1.0;
        }
        let year = self.current_period % 12;
        calculations::calculate_avg_points_factor(year)
        // as f64;
        //[1,5..1.0] in 40 years
        //1.0+(40+1)^2/40/40*0,5
        //let result = 1.0 + (((40.0 + 1.0 - years) * (40.0 + 1.0 - years)) / 40.0) / 40.0 * 0.5;
        //result
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

    #[test]
    fn calculate_points_should_be_one() {
        let mut pension = Pension::new();
        pension.settings.current_contribution_value = 10.0;
        let result_one = pension.calculate_points(10.0, 1.0, 100.0);
        assert_eq!(result_one, 1.0);
    }

    #[test]
    fn calculate_avg_points_should_be_one_five_to_one() {
        let mut pension = Pension::new();
        pension.current_period = 1;
        let result_one_five = pension.calculate_avg_points();
        println!("{}", result_one_five);
        assert_eq!(result_one_five, 1.5f64);

        pension.current_period = 40 * 12;
        let result_one_five = pension.calculate_avg_points();
        assert_eq!(result_one_five, 1.0f64);
    }
}
