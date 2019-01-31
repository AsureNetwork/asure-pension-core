use crate::new::types::*;
use std::collections::HashMap;
use crate::new::contributor::Contributor;
use crate::new::user::User;
use crate::new::pensioner::Pensioner;
use crate::calculations::*;
use std::cmp::Ordering;

pub struct PeriodState {
    current_dpt_bonus: Dpt,
    current_contribution_value: Unit,
    current_contribution_value_degree: Unit,
    max_contribution_value: Unit,

    monthly_dpt_unit_rate: Option<Result<f64, String>>,
    savings_dpt_unit_rate: Option<Result<f64, String>>,

    contributions: Vec<Unit>,
    contributions_total: Unit,
    contributions_avg: Unit,

    pensions_total: Unit,
}

impl PeriodState {
    pub fn new() -> Self {
        PeriodState {
            current_dpt_bonus: 0.5,
            current_contribution_value: 1.0,
            current_contribution_value_degree: 10.0,
            max_contribution_value: 1.0,

            monthly_dpt_unit_rate: None,
            savings_dpt_unit_rate: None,

            contributions: vec![],
            contributions_total: 0.0,
            contributions_avg: 0.0,

            pensions_total: 0.0,
        }
    }
}

pub struct Pension {
    pub period: Period,
    period_states: HashMap<Period, PeriodState>,

    contributions_total: Unit,
    periods_open: Period,

    dpt_total: Dpt,
    dpt_pensioner: Dpt,
    dpt_done: Dpt,
}

impl Pension {
    pub fn new() -> Self {
        Pension {
            period: 0,
            period_states: HashMap::new(),

            contributions_total: 0.0,
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
        self.periods_open += 480;
    }

    pub fn contribute(&mut self, contributor: &mut Contributor, contribution: Unit) -> Result<(), String> {
        contributor.contribute(contribution, self.period)?;

        let mut state = self.period_state_mut();
        state.contributions.push(contribution);
        state.contributions_total += contribution;
        state.contributions_avg = state.contributions_total / state.contributions.len() as Unit;

        self.contributions_total += contribution;

        Ok(())
    }

    pub fn retire(&mut self, contributor: Contributor) -> User {
        self.dpt_pensioner += contributor.dpts.values().map(|dpt| dpt).sum::<Dpt>();

        contributor.retire()
    }

    pub fn try_finish(&mut self, pensioner: Pensioner) -> User {
        let user = pensioner.try_finish();

        match &user {
            User::Done(done_user) => {
                self.dpt_done = done_user.pensioner.contributor.dpt_total();
            }
            _ => ()
        }

        user
    }

    pub fn prepare_claim_dpt(&mut self, users: &[User]) -> Result<(), String> {
        let period = self.period;
        let mut state = self.period_state_mut();

        let contributions = users
            .iter()
            .filter_map(|user| user.to_contributor())
            .filter_map(|contributor| contributor.contributions.get(&period))
            .map(|contribution| *contribution)
            .collect::<Vec<_>>();

        // TODO: Is this correct?
        if contributions.len() == 0 {
            return Ok(());
        }

        state.current_dpt_bonus = calculate_dpt_bonus_by_period(period);
        state.current_contribution_value = calculate_contribution_value(
            state.current_contribution_value,
            state.current_contribution_value_degree,
            &contributions,
        );

        let mut sorted_period_amounts: Vec<f64> = contributions.to_vec();
        sorted_period_amounts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        state.max_contribution_value = *sorted_period_amounts.last().unwrap();

        Ok(())
    }

    pub fn claim_dpt(&mut self, contributor: &mut Contributor) -> Result<(), String> {
        if let Some(contribution) = contributor.contributions.get(&self.period) {
            let state = self.period_state();

            let dpt = calculate_dpt(
                *contribution,
                state.current_contribution_value,
                state.current_dpt_bonus,
                state.max_contribution_value,
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

        let period = self.period;
        let state = self.period_state();

        let monthly_dpt_unit_rate = state.monthly_dpt_unit_rate.clone().unwrap();
        let savings_dpt_unit_rate = state.savings_dpt_unit_rate.clone().unwrap();

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
                        pensioner.claim_pension(period, savings_dpt_unit_rate);
                    }
                    0.0
                }
                Err(_) => pensioner.claim_pension(period, savings_dpt_unit_rate)
            };
        }

        let state = self.period_state_mut();
        state.pensions_total += pension;

        self.periods_open -= 1;
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

        let active_users_dpt = vec![self.dpt_total - self.dpt_done];
        let total_open_months = self.periods_open as f64;

        let savings_dpt_unit_rate = if total_open_months <= 0.0 {
            Some(Err("no total_open_months in period".to_string()))
        } else if self.contributions_total <= 0.0 {
            Some(Err("no contributions in period".to_string()))
        } else {
            Some(Ok(calculate_savings_dpt_unit_rate(
                &active_users_dpt, total_open_months, self.contributions_total,
            )))
        };

        let state = self.period_state_mut();
        state.savings_dpt_unit_rate = savings_dpt_unit_rate;
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