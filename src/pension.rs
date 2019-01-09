//use std::mem;
//use std::cell::RefCell;

use crate::common::*;
use crate::period::*;
use crate::user::*;
use crate::transaction::*;

pub struct Pension {
    pub period_index: u64,
    pub period: Vec<Period>,
    pub total_eth: f64,
    pub total_month_eth: f64,
    pub total_dpt: f64,
    pub total_month_dpt: f64,
    pub total_retirement_dpt: f64,
    pub users: Vec<User>,
    pub current_period: Period,
    pub settings: Settings,
}

struct PensionFold {
    txs: Vec<Transaction>,
    total_eth: f64,
    total_month_eth: f64,
}

impl PensionFold {
    pub fn new() -> PensionFold {
        PensionFold {
            txs: vec![],
            total_eth: 0.0,
            total_month_eth: 0.0,
        }
    }
}

impl Pension {
    pub fn new() -> Pension {
        Pension {
            period_index: 0,
            period: Vec::new(),
            total_eth: 0.0,
            total_month_eth: 0.0,
            total_dpt: 0.0,
            total_month_dpt: 0.0,
            total_retirement_dpt: 0.0,
            users: Vec::new(),
            current_period: Period::new(),
            settings: Settings::new(),
        }
    }

    pub fn create_users(&mut self, count: u32) {
        for _ in 0..count {
            self.users.push(User::new());
        }
    }

    pub fn start(&mut self) {
        self.period_index += 1;
        self.current_period = Period::new();
    }

    pub fn pay(&mut self) {
        self.total_month_eth = 0.0;

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
                tx.amount = 20.0;

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
        let price = self.settings.current_contribution_value;
        let result = match amount {
            _ if amount > price =>
                (1f64 + (amount - price) / (max - price)) * self.settings.current_avg_points,
            _ if amount < price =>
                ((amount - min) / (price - min)) * self.settings.current_avg_points,
            _ => 1f64,
        };

        result
    }

    pub fn end(&self) {
        if self.current_period.txs.is_empty() {
            return;
        }

        let plus = self.current_period.txs
            .iter()
            .filter(|tx| tx.amount > self.settings.current_contribution_value)
            .count();
    }

    pub fn calculate_avg_points(&self) -> f64 {
        assert_ne!(self.period_index, 0);
        if self.period_index >= 40 * 12 {
            return 1.0;
        }
        let years = (self.period_index % 12) as f64;
        //[1,5..1.0] in 40 years
        //1.0+(40+1)^2/40/40*0,5
        let result = 1.0 + (((40.0 + 1.0 - years) * (40.0 + 1.0 - years)) / 40.0) / 40.0 * 0.5;
        result
    }
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
        pension.period_index = 1;
        let result_one_five = pension.calculate_avg_points();
        println!("{}", result_one_five);
        assert_eq!(result_one_five, 1.5f64);

        pension.period_index = 40 * 12;
        let result_one_five = pension.calculate_avg_points();
        assert_eq!(result_one_five, 1.0f64);
    }

}
