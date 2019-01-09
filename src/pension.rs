//use std::mem;
//use std::cell::RefCell;
//use std::iter::FromIterator;

use crate::common::*;
use crate::user::*;


pub struct Pension {
    pub total_eth: f64,
    pub total_month_eth: f64,
    pub total_dpt: f64,
    pub total_month_dpt: f64,
    pub total_retirement_dpt: f64,
    pub users: Vec<User>,
    pub current_period: u32,
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
    }

    pub fn pay(&mut self) {
        self.total_month_eth = 0.0;

        let period = self.current_period;

        let result = self.users
            .iter_mut()
            .filter(|u| u.pension_status == PensionStatus::Run)
            .fold(PensionFold::new(), |mut state, user| {

                if user.pension_payment_months == 480 {
                    user.activate_retirement();
                    return state;
                }

                let amount = 20.0;
                user.pay_into_pension(period, amount);

                state.total_eth += amount;
                state.total_month_eth += amount;

                return state;
            });

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

        //let period = self.current_period;

        let _all_txs = self.users
            .iter()
            .flat_map(|user| &user.transactions)
            .filter(|tx| tx.period == self.current_period);

//        for tx in all_txs {
//
//        }

//        let mut txs = Vec::from_iter(
//            self.users.iter()
//            .filter_map(|u| u.transactions.iter().)
//            .cloned());
//
//
//        let plus = CurrentPeriod.Txs.Count(t => t.Amount > Settings.currentPrice);
//        let minus = CurrentPeriod.Txs.Count(t => t.Amount < Settings.currentPrice);
//
//        var sum = CurrentPeriod.Txs.Sum(t => t.Amount);
//        var average = CurrentPeriod.Txs.Average(t => t.Amount);
//        var max = CurrentPeriod.Txs.Max(t => t.Amount);
//        var min = CurrentPeriod.Txs.Min(t => t.Amount);
//
//
//        Logging("plus:" + plus);
//        Logging("minus:" + minus);
//
//        Logging("sum:" + sum);
//        Logging("average:" + average);
//        Logging("max:" + max);
//        Logging("min:" + min);
//        this.TotalMonthDpt = 0;
//        foreach (var user in Users)
//        {
//            var tx = CurrentPeriod.Txs.FirstOrDefault((t) => t.User == user);
//            if (tx != null)
//                {
//                    var amount = CalculatePoints(tx.Amount, min, max);
//                    Settings.Tokens += amount;
//                    var token = new Token()
//                    {
//                        Created = CurrentPeriod.Date,
//                        Amount = amount
//                    };
//                    this.TotalMonthDpt += amount;
//                    user.Wallet.Tokens.Add(token);
//                    Logging("User:" + user.Name + ":" + token.Amount + " PT");
//                }
//        }
//        Settings.currentPrice = plus > minus ? Settings.currentPrice * (1.0 + Settings.currentPointsDegree / 100) : Settings.currentPrice *(1 - Settings.currentPointsDegree / 100);
//        Logging("Settings.currentPrice:" + Settings.currentPrice);
//        Logging("Settings.ETH:" + Settings.ETH);
//        Logging("Settings.Tokens:" + Settings.Tokens);
//        Logging("");

    }

    pub fn calculate_avg_points(&self) -> f64 {
        assert_ne!(self.current_period, 0);
        if self.current_period >= 40 * 12 {
            return 1.0;
        }
        let years = (self.current_period % 12) as f64;
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
        pension.current_period = 1;
        let result_one_five = pension.calculate_avg_points();
        println!("{}", result_one_five);
        assert_eq!(result_one_five, 1.5f64);

        pension.current_period = 40 * 12;
        let result_one_five = pension.calculate_avg_points();
        assert_eq!(result_one_five, 1.0f64);
    }

}
