#![allow(dead_code)]


use std::cmp::Ordering::Equal;

use crate::csvexport::PensionCsvExporter;
use crate::settings::*;
use crate::user::*;

//use std::cell::RefCell;
//use std::rc::Rc;
//pub mod pension;

pub mod calculations;
pub mod csvexport;
pub mod transaction;
pub mod user;
pub mod wallet;
pub mod token;
pub mod settings;


//use std::mem;
//use std::cell::RefCell;
//use std::iter::FromIterator;
#[derive(Debug)]
pub struct Pension {
    pub current_dpt_bonus: f64,
    pub total_eth: f64,
    pub total_month_eth: f64,
    pub total_dpt: f64,
    pub total_month_dpt: f64,
    pub total_retirement_dpt: f64,
    pub users: Vec<User>,
    pub current_period: u64,
    pub settings: Settings,
}

pub trait PensionSimulation {
    fn name(&mut self) -> String;
    fn create_user(&mut self, current_period: u64) -> u32;
    fn should_retire(&mut self, contributor: &User) -> bool;
    fn pay_pension(&mut self, contributor: &User) -> Option<f64>;
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
            current_dpt_bonus: 0.5,
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

    pub fn simulate<T>(mut simulation: T) where T: PensionSimulation {
        println!("Pension {}", simulation.name());

        let mut pension = Pension::new();
        let mut pension_exporter = PensionCsvExporter::new();

        loop {
            pension.start();
            pension.create_users(simulation.create_user(pension.current_period));

            let mut contributors = pension.users
                .iter_mut()
                .filter(|contributor| contributor.pension_status == PensionStatus::Run)
                .collect::<Vec<_>>();

            contributors
                .iter_mut()
                .filter(|contributor| simulation.should_retire(contributor))
                .for_each(|contributor| {
                    contributor.activate_retirement();
                });


            let contributor_payments = contributors
                .iter()
                .filter_map(|contributor| simulation.pay_pension(contributor))
                .collect::<Vec<_>>();

            let current_period = pension.current_period;
            let total_payments = contributors
                .iter_mut()
                .zip(contributor_payments)
                .fold(0.0, |total_payments, (contributor, payment)| {
                    match contributor.pay(current_period, payment) {
                        Ok(()) => total_payments + payment,
                        Err(_) => total_payments
                    }
                });

            pension.add_amount(total_payments);

            pension.payout();
            pension.end();

            pension.users
                .iter_mut()
                .filter(|user| user.is_pension_payment_complete())
                .for_each(|user| user.pension_status = PensionStatus::Done);

            pension.print();
            pension_exporter.add_pension(&pension);
            pension_exporter.add_users(&pension);

            if pension.users.iter().all(|user| user.pension_status == PensionStatus::Done) {
                break;
            }
        }

        println!("{:?}", pension);

        pension_exporter.export_pensions(format!("{}-pensions.csv", simulation.name().to_lowercase()));
        pension_exporter.export_users(format!("{}-users.csv", simulation.name().to_lowercase()));
    }

    pub fn print(&self) {
        let contributor_count = self.users.iter().filter(|user| user.pension_status == PensionStatus::Run).count();
        let pensioner_count = self.users.iter().filter(|user| user.pension_status == PensionStatus::Retirement).count();
        let done_count = self.users.iter().filter(|user| user.pension_status == PensionStatus::Done).count();
        let total_pension_eth: f64 = self.users.iter().map(|user| user.wallet.pension_eth).sum();

        println!("Period: {}, Total Eth: {}, Total Pension Eth: {}, Total DPT: {}, Total Contributor: {}, Total Pensioner: {}, Total Done: {}",
                 self.current_period, self.total_eth, total_pension_eth, self.total_dpt, contributor_count, pensioner_count, done_count);
        for user in &self.users {
            println!("User: {}, Status: {:?}, Wallet: {}, Pension: {}, Pension Months Allowed: {}, Pensions Months Received: {}, DPT: {} + ({})",
                     user.id, user.pension_status, user.wallet.eth, user.wallet.pension_eth, user.allowed_pension_receive_months(),
                     user.pension_receive_months, user.wallet.dpt.amount, user.last_dpt);
        }

        println!();
        println!("-------------------------");
        println!();
    }

    pub fn create_users(&mut self, count: u32) {
        for _ in 0..count {
            self.users.push(User::new());
        }
    }

    pub fn start(&mut self) {
        self.current_period += 1;
        self.total_month_eth = 0.0;
        self.current_dpt_bonus = calculations::calculate_dpt_bonus_by_period(self.current_period);
    }

    pub fn add_amount(&mut self, amount: f64) {
        self.total_eth += amount;
        self.total_month_eth += amount;
    }

    pub fn payout(&mut self) {
        // If there are no pensioners in the current month, we don't have to
        // payout anything and return early.
        if self.pension_dpt_total() == 0.0 {
            return ();
        }

        let contributions_month = self.contributions_of_current_period();
        let contributions_month_total: f64 = contributions_month.iter().sum();
        let contributions_month_avg = contributions_month_total / contributions_month.len() as f64;

        // Payout all_eth_month of the current month to all pensioners
        let mut monthly_dpt_unit_rate = 0.0;
        if contributions_month_total > 0.0 {
            monthly_dpt_unit_rate = self.calculate_monthly_dpt_unit_rate();
            self.payout_monthly_contributions(480.0 * monthly_dpt_unit_rate);
        }

        // Payout parts of the saved all_eth_month of previous month to all pensioners
        if self.total_eth > 0.0 && (contributions_month_total == 0.0 || monthly_dpt_unit_rate < contributions_month_avg) {
            let savings_dpt_unit_rate = self.calculate_savings_dpt_unit_rate();
            self.payout_saved_contributions(savings_dpt_unit_rate);
        }
    }

    fn contributions_of_current_period(&self) -> Vec<f64> {
        let period = self.current_period;
        self.users
            .iter()
            .filter(|user| user.pension_status == PensionStatus::Run)
            .flat_map(|user| &user.transactions)
            .filter(|tx| tx.period == period)
            .map(|tx| tx.amount)
            .collect::<Vec<_>>()
    }

    fn pension_dpt_total(&self) -> f64 {
        self.users
            .iter()
            .filter(|user| user.pension_status == PensionStatus::Retirement)
            .map(|user| user.wallet.dpt.amount)
            .sum()
    }

    fn payout_monthly_contributions(&mut self, monthly_dpt_unit_rate: f64) {
        let pensions_from_month = self.users
            .iter_mut()
            .filter(|user| user.pension_status == PensionStatus::Retirement)
            .fold(0.0, |acc, user| {
                let amount_unit = user.wallet.dpt.amount / monthly_dpt_unit_rate; // / 480.0
                user.wallet.pension_eth += amount_unit;

                return acc + amount_unit;
            });

        self.total_eth -= pensions_from_month;
        assert!(self.total_eth >= 0.0, "self.total_eth: {}", self.total_eth);
    }

    fn payout_saved_contributions(&mut self, savings_dpt_unit_rate: f64) {
        let pensions_from_savings = self.users
            .iter_mut()
            .filter(|user| user.pension_status == PensionStatus::Retirement)
            .fold(0.0, |acc, user| {
                let amount_unit = user.wallet.dpt.amount * savings_dpt_unit_rate;

                match user.payout(amount_unit) {
                    Ok(()) => acc + amount_unit,
                    Err(err) => panic!(err),
                }
            });

        self.total_eth -= pensions_from_savings;
        assert!(self.total_eth >= 0.0, "self.total_eth: {}", self.total_eth);
        ()
    }

    fn calculate_monthly_dpt_unit_rate(&mut self) -> f64 {
        let contributions_month = self.contributions_of_current_period();
        let pension_dpt_total = self.pension_dpt_total();

        calculations::calculate_monthly_dpt_unit_rate(&contributions_month, pension_dpt_total)
    }

    fn calculate_savings_dpt_unit_rate(&self) -> f64 {
        let active_users = self.users
            .iter()
            .filter(|user| user.pension_status != PensionStatus::Done)
            .collect::<Vec<_>>();
        let total_open_months = self.total_open_months();

        calculations::calculate_savings_dpt_unit_rate(&active_users, total_open_months, self.total_eth)
    }

    fn total_open_months(&self) -> f64 {
        self.users
            .iter()
            .map(|user| {
                match user.pension_status {
                    PensionStatus::Run => 480.0,
                    PensionStatus::Retirement => user.allowed_pension_receive_months() as f64 - user.pension_receive_months as f64,
                    PensionStatus::Done => 0.0
                }
            })
            .sum()
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
                    self.current_dpt_bonus,
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
