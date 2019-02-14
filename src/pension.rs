//use std::cmp::Ordering;
use std::collections::HashMap;

use crate::calculations::*;
use crate::contributor::Contributor;
use crate::pensioner::Pensioner;
use crate::types::*;
use crate::user::User;

pub struct PeriodState {
    current_dpt_bonus: Dpt,
    current_contribution_value: Unit,
    //max_contribution_value: Unit,

    monthly_dpt_unit_rate: Option<Result<f64, String>>,
    savings_dpt_unit_rate: Option<Result<f64, String>>,
    laggards_dpt_unit_rate: Option<Result<f64, String>>,

    contributions: Vec<Unit>,
    contributions_total: Unit,
    contributions_avg: Unit,
    laggards_total: Unit,

    pensions_total: Unit,

}

impl PeriodState {
    pub fn new() -> Self {
        PeriodState {
            current_dpt_bonus: 1.5,
            current_contribution_value: 1.0,
            //max_contribution_value: 1.0,

            monthly_dpt_unit_rate: None,
            savings_dpt_unit_rate: None,
            laggards_dpt_unit_rate: None,

            contributions: vec![],
            contributions_total: 0.0,
            contributions_avg: 0.0,
            laggards_total: 0.0,

            pensions_total: 0.0,

        }
    }
}

pub struct Pension {
    pub(super) period: Period,
    period_states: HashMap<Period, PeriodState>,

    pub(super) current_contribution_value: Unit,
    current_contribution_value_degree: Unit,

    contributors_total: u64,
    pensioners_total: u64,
    done_users_total: u64,

    pub(super) savings_total: Unit,
    pub(super) contributions_total: Unit,
    pub(super) pensions_total: Unit,
    pub(super) laggards_total: Unit,
    periods_open: Period,

    pub(super) dpt_total: Dpt,
    dpt_pensioner: Dpt,
    dpt_done: Dpt,
}

impl Pension {
    pub fn new() -> Self {
        Pension {
            period: 0,
            period_states: HashMap::new(),

            current_contribution_value: 1.0,
            current_contribution_value_degree: 10.0,

            contributors_total: 0,
            pensioners_total: 0,
            done_users_total: 0,

            savings_total: 0.0,
            contributions_total: 0.0,
            pensions_total: 0.0,
            laggards_total: 0.0,
            periods_open: 0,

            dpt_total: 0.0,
            dpt_pensioner: 0.0,
            dpt_done: 0.0,
        }
    }

    pub fn period(&self) -> Period {
        self.period
    }

    pub fn start_new_period(&mut self) {
        self.period += 1;
        self.period_states.insert(self.period, PeriodState::new());
    }

    pub fn join(&mut self, _contributor: &Contributor) {
        self.contributors_total += 1;
        self.periods_open += 480;
    }

    pub fn contribute(&mut self, contributor: &mut Contributor, contribution: Unit) -> Result<(), String> {
        contributor.contribute(contribution, self.period)?;

        self.contributions_total += contribution;
        self.savings_total += contribution;

        let mut state = self.period_state_mut();
        state.contributions.push(contribution);

        if contributor.has_retire_months() {
            state.contributions_total += contribution;
            state.contributions_avg = state.contributions_total / state.contributions.len() as Unit;
        } else {
            state.laggards_total += contribution;
            self.laggards_total += contribution;
        }

        Ok(())
    }
    //todo contributor join();
    pub fn retire(&mut self, contributor: Contributor) -> User {
        self.dpt_pensioner += contributor.dpts.values().map(|dpt| dpt).sum::<Dpt>();
        self.contributors_total -= 1;
        self.pensioners_total += 1;
        self.periods_open -= 480 - contributor.allowed_pension_periods();
        contributor.retire()
    }

    pub fn try_finish(&mut self, pensioner: Pensioner) -> User {
        let user = pensioner.try_finish();

        match &user {
            User::Done(done_user) => {
                self.dpt_done += done_user.pensioner.contributor.dpt_total();
                self.pensioners_total -= 1;
                self.done_users_total += 1;
            }
            _ => ()
        }

        user
    }

    pub fn prepare_claim_dpt(&mut self, users: &[User]) -> Result<(), String> {
        let period = self.period;

        let contributions = users
            .iter()
            .filter_map(|user| user.to_contributor())
            .filter_map(|contributor| contributor.contributions.get(&period))
            .map(|contribution| *contribution)
            .collect::<Vec<_>>();


        let current_dpt_bonus = calculate_dpt_bonus_by_period(period);
        let current_contribution_value = calculate_contribution_value(
            self.current_contribution_value,
            self.current_contribution_value_degree,
            &contributions,
        );
        self.current_contribution_value = current_contribution_value;

        let mut state = self.period_state_mut();
        state.current_dpt_bonus = current_dpt_bonus;
        state.current_contribution_value = current_contribution_value;

//        if contributions.len() > 0 {
//            let mut sorted_period_amounts: Vec<f64> = contributions.to_vec();
//            sorted_period_amounts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
//            state.max_contribution_value = *sorted_period_amounts.last().unwrap();
//        }

        Ok(())
    }

    pub fn claim_dpt(&mut self, contributor: &mut Contributor) -> Result<(), String> {
        if let Some(contribution) = contributor.contributions.get(&self.period) {
            let state = self.period_state();

            let dpt = calculate_dpt(
                *contribution,
                state.current_contribution_value,
                state.current_dpt_bonus,
                //state.max_contribution_value,
            );

            contributor.claim_dpt(dpt, self.period)?;

            self.dpt_total += dpt;
        }

        Ok(())
    }

    pub fn claim_pension(&mut self, pensioner: &mut Pensioner) -> Result<(), String> {
        if pensioner.contributor.dpt_total() <= 0.0 {
            // TODO: What should happen if the pensioner has no dpt?
            //       Maybe we should return an Err?
            return Ok(());
        }

        // The first pensioner who claims his/her pension has to
        // calculate monthly_dpt_unit_rate and savings_dpt_unit_rate
        // for the current period.
        self.calculate_monthly_dpt_unit_rate();
        self.calculate_savings_dpt_unit_rate();
        self.calculate_laggards_dpt_unit_rate();

        let period = self.period;
        let state = self.period_state();

        let monthly_dpt_unit_rate = state.monthly_dpt_unit_rate.clone().unwrap();
        let savings_dpt_unit_rate = state.savings_dpt_unit_rate.clone().unwrap();
        let laggards_dpt_unit_rate = state.laggards_dpt_unit_rate.clone().unwrap();

        let mut pension = 0.0;
        // Redistribute contributions of current period if available
        // (Hint: if the rate is valid, then some contributions happened).
        if let Ok(monthly_dpt_unit_rate) = monthly_dpt_unit_rate {
            pension = pensioner.claim_pension(period, monthly_dpt_unit_rate);
        }

        // Payout parts of the saved all_eth_month of previous month to all pensioners
        if let Ok(savings_dpt_unit_rate) = savings_dpt_unit_rate {
            pension = match monthly_dpt_unit_rate {
                Ok(monthly_dpt_unit_rate) => {
                    if monthly_dpt_unit_rate < state.contributions_avg {
                        pensioner.claim_pension(period, savings_dpt_unit_rate)
                    } else {
                        pension
                    }
                }
                Err(_) => pensioner.claim_pension(period, savings_dpt_unit_rate)
            };
        }

        //payout laggards_dpt_unit_rate
        if let Ok(laggards_dpt_unit_rate) = laggards_dpt_unit_rate {
            pension = pensioner.claim_pension(period, laggards_dpt_unit_rate);
        }

        let state = self.period_state_mut();
        state.pensions_total += pension;

        self.pensions_total += pension;
        self.savings_total -= pension;
        self.periods_open -= 1;

        //    assert!(self.contributions_total - self.pensions_total >= 0.0,
        //            "self.contributions_total {} - self.pensions_total {} = {}",
        //            self.contributions_total, self.pensions_total, self.contributions_total - self.pensions_total);

        Ok(())
    }

    fn calculate_monthly_dpt_unit_rate(&mut self) {
        let state = self.period_state();

        if state.monthly_dpt_unit_rate.is_some() {
            return ();
        }

        let contributions_month = &state.contributions;
        let pension_dpt_total = self.dpt_pensioner;

        let monthly_dpt_unit_rate = if contributions_month.len() == 0 {
            Some(Err("no contributions in period".to_string()))
        } else if pension_dpt_total <= 0.0 {
            Some(Err("no pension_dpt_total in period".to_string()))
        } else {
            Some(Ok(calculate_monthly_dpt_unit_rate(
                &contributions_month, pension_dpt_total,
            )))
        };

        let state = self.period_state_mut();
        state.monthly_dpt_unit_rate = monthly_dpt_unit_rate;
    }

    fn calculate_savings_dpt_unit_rate(&mut self) {
        let state = self.period_state();
        if state.savings_dpt_unit_rate.is_some() {
            return ();
        }

        let active_users_count = self.contributors_total + self.pensioners_total;
        let active_users_dpt = self.dpt_total - self.dpt_done;
        let total_open_months = self.periods_open as f64;

        let savings_dpt_unit_rate = if total_open_months <= 0.0 {
            Some(Err("no total_open_months in period".to_string()))
        } else if self.savings_total <= 0.0 {
            Some(Err("no contributions in period".to_string()))
        } else {
            let savings_redistribution_part = self.savings_total - self.laggards_total;
            Some(Ok(calculate_savings_dpt_unit_rate(
                active_users_count, active_users_dpt, total_open_months,
                savings_redistribution_part,
            )))
        };

        let state = self.period_state_mut();
        state.savings_dpt_unit_rate = savings_dpt_unit_rate;
    }

    fn calculate_laggards_dpt_unit_rate(&mut self) {
        let state = self.period_state();
        if state.laggards_dpt_unit_rate.is_some() {
            return ();
        }

        let open_period_rate = self.pensioners_total / self.periods_open;
        if open_period_rate == 1 {
            let active_users_count = self.contributors_total + self.pensioners_total;
            let active_users_dpt = self.dpt_total - self.dpt_done;
            let total_open_months = self.periods_open as f64;

            let laggards_dpt_unit_rate = if total_open_months <= 0.0 {
                Some(Err("no total_open_months in period".to_string()))
            } else if self.laggards_total <= 0.0 {
                Some(Err("no laggards in period".to_string()))
            } else {
                Some(Ok(calculate_laggards_dpt_unit_rate(
                    active_users_count, active_users_dpt, total_open_months, self.laggards_total,
                )))
            };

            let state = self.period_state_mut();
            state.laggards_dpt_unit_rate = laggards_dpt_unit_rate;
        } else {
            let state = self.period_state_mut();
            state.laggards_dpt_unit_rate = Some(Err("no rate".to_string()));
        }
    }

    fn period_state(&self) -> &PeriodState {
        match self.period_states.get(&self.period) {
            Some(state) => state,
            None => panic!(format!("No state for period {} found", self.period))
        }
    }

    fn period_state_mut(&mut self) -> &mut PeriodState {
        match self.period_states.get_mut(&self.period) {
            Some(state) => state,
            None => panic!(format!("No state for period {} found", self.period))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pension::Pension;
    use crate::contributor::Contributor;
    //use std::collections::HashMap;
    //use crate::types::Dpt;

    #[test]
    fn start_new_period() {
        let mut pension = Pension::new();
        pension.start_new_period();
        assert_eq!(pension.period, 1);
    }

    #[test]
    fn join() {
        let mut pension = Pension::new();
        let mut contributor = Contributor::new();
        contributor.contribute(1.0, 1).unwrap();
        ;
        pension.join(&contributor);
        assert_eq!(pension.contributors_total, 1);
        assert_eq!(pension.periods_open, 480);
    }

    #[test]
    fn retire() {
        let mut pension = Pension::new();
        let mut contributor = Contributor:: new();
        pension.join(&contributor);
        for period in 1..241 {
            contributor.contribute(1.0, period).unwrap();
        }
        pension.retire(contributor);
        assert_eq!(pension.pensioners_total, 1);
        assert_eq!(pension.contributors_total, 0);
        assert_eq!(pension.periods_open, 120);
    }
}