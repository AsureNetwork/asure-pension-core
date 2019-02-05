//macro_rules! log {
//    ($msg:expr) => {{
//        let state: i32 = get_log_state();
//        if state > 0 {
//            println!("log({}): {}", state, $msg);
//        }
//    }};
//}

pub const MIN_POSITIVE: f64 = 0.0000000000000001;//std::f64::MIN_POSITIVE

pub fn avg(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

//    pub fn median(numbers: &mut [f64]) -> f64 {
//        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
//        let mid = numbers.len() / 2;
//        numbers[mid]
//    }

pub fn calculate_contribution_value(contribution_value: f64,
                                    contribution_value_degree: f64,
                                    numbers: &[f64]) -> f64 {
    let nums = numbers.to_vec();
    let ref_value = self::avg(&nums);
    let ccv = contribution_value;
    let diff = ((ref_value.max(ccv) - ref_value.min(ccv)) / ref_value.max(ccv)) * 100.0;

    //println!("ref_value {},ccv {} = diff {} %", ref_value, ccv, diff);
    if diff > contribution_value_degree {
        let factor = contribution_value_degree / 100.0;
        if ref_value > contribution_value {
            return contribution_value * (1.0 + factor);
        } else {
            return contribution_value * (1.0 - factor);
        }
    }
    return contribution_value;
}

pub fn calculate_dpt(amount: f64,
                     contribution_value: f64,
                     dpt_bonus: f64,
                     max: f64) -> f64 {
    assert!(dpt_bonus >= 1.0 && dpt_bonus <= 1.5);
    assert!(max >= amount);

    let ccv = contribution_value;

    if amount > ccv {
        return (1.0 + (amount - ccv) / (max - ccv)) * dpt_bonus;
    }
    if amount < ccv {
        return ((amount - MIN_POSITIVE) / (ccv - MIN_POSITIVE)) * dpt_bonus;
    }
    1f64 * dpt_bonus //amount == ccv
}

pub fn calculate_entitlement_months(periods: u64) -> u64 {
    return (periods * periods) / 480;
}

pub fn calculate_dpt_bonus_by_period(period: u64) -> f64 {
    assert_ne!(period, 0);
    if period >= 40 * 12 {
        return 1.0;
    }
    let year = (period / 12) + 1;
    calculate_dpt_bonus(year)
}

pub fn calculate_dpt_bonus(year: u64) -> f64 {
    assert_ne!(year, 0);
    if year >= 40 {
        return 1.0;
    }
    let y = year as f64;
    //[1,5..1.0] in 40 years
    //1.0+(40+1)^2/40/40*0,5
    let result = (((40.0 + 1.0 - y).powf(2.0)) / 1600f64) * 0.5;
    result + 1.0
}

pub fn calculate_monthly_dpt_unit_rate(contributions_month: &[f64], pension_dpt_total: f64) -> f64 {
    assert!(contributions_month.len() > 0, "contributions_month must be greater than zero");
    assert!(pension_dpt_total > 0.0, "pension dpt total must be greater than zero");
    let contributions_month_total: f64 = contributions_month.iter().sum();
    let contributions_month_avg = contributions_month_total / contributions_month.len() as f64;

    let monthly_dpt_unit_rate =
        contributions_month_total / (pension_dpt_total * contributions_month_avg);

    floor((contributions_month_avg / 480.0).min(monthly_dpt_unit_rate))
}

pub fn calculate_savings_dpt_unit_rate(active_users_count: u64, active_users_dpt: f64, total_open_months: f64, total_unit: f64) -> f64 {
    assert!(active_users_count > 0, "active_users must be greater than zero");
    assert!(active_users_dpt > 0.0, "active_users_dpt must be greater than zero");
    assert!(total_open_months > 0.0, "total_open_months must be greater than zero");

    let avg_open_months = total_open_months / active_users_count as f64;

    floor(total_unit / (active_users_dpt * avg_open_months))
}

pub fn calculate_laggards_dpt_unit_rate(active_users_count: u64, active_users_dpt: f64, total_open_months: f64, total_unit: f64) -> f64 {
    assert!(active_users_count > 0, "active_users must be greater than zero");
    assert!(active_users_dpt > 0.0, "active_users_dpt must be greater than zero");
    assert!(total_open_months > 0.0, "total_open_months must be greater than zero");

    let avg_open_months = total_open_months / active_users_count as f64;

    floor(total_unit / (active_users_dpt * avg_open_months))
}


pub fn floor(number: f64) -> f64 {
    number.floor_factor(16)
}

pub trait Floor {
    fn floor_factor(&self, number: u32) -> f64;
}

impl Floor for f64 {
    fn floor_factor(&self, number: u32) -> f64 {
        let factor = (10_u64).pow(number) as f64;
        ((*self) * factor).floor() / factor
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::*;

    #[test]
    fn test_calculate_savings_dpt_unit_rate() {
        let users_dpt = [480.0, 480.0, 480.0, 480.0, 480.0,
            480.0, 480.0, 480.0, 480.0, 480.0];
        let total_open_months = 480.0 * 10.0;
        let total_eth = 480.0 * 10.0;

        let result = calculate_savings_dpt_unit_rate(users_dpt.len() as u64,
                                                     users_dpt.iter().sum(),
                                                     total_open_months,
                                                     total_eth);
        assert_eq!(result, floor(1.0 / 480.0));
    }


    #[test]
    fn test_calculate_monthly_dpt_unit_rate() {
        let contributions_month = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pension_dpt_total = 480.0 * 10.0;

        let result = calculate_monthly_dpt_unit_rate(&contributions_month, pension_dpt_total);

        assert_eq!(result, floor(1.0 / 480.0));
    }

    #[test]
    fn test_calculate_monthly_dpt_unit_rate_by_twenty() {
        let contributions_month = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pension_dpt_total = 480.0 * 20.0;

        let result = calculate_monthly_dpt_unit_rate(&contributions_month, pension_dpt_total);

        assert_eq!(result, floor(1.0 / 960.0));
    }

    #[test]
    fn test_calculate_monthly_dpt_unit_rate_by_five() {
        let contributions_month = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let pension_dpt_total = 480.0 * 5.0;

        let result = calculate_monthly_dpt_unit_rate(&contributions_month, pension_dpt_total);

        assert_eq!(result, floor(1.0 / 480.0));
    }

    #[test]
    #[should_panic]
    fn test_calculate_monthly_dpt_unit_rate_by_zero() {
        calculate_monthly_dpt_unit_rate(&[], 0.0);
    }

    #[test]
    fn diff() {
        let ref_value: f64 = 1.0;
        let ccv: f64 = 1.0;
        let diff = ((ref_value.max(ccv) - ref_value.min(ccv)) / ref_value.max(ccv)) * 100.0;

        assert_eq!(diff, 0.0);
    }

    #[test]
    fn avg() {
        let numbers = [1.0, 0.0, 5.0];
        let result = super::avg(&numbers);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn calculate_entitlement_months() {
        let mut result = super::calculate_entitlement_months(12);
        assert_eq!(result, 0);
        result = super::calculate_entitlement_months(22);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_calculate_contribution_value() {
        let mut numbers = [1.0, 1.0, 1.0];
        let contribution_value = 1.0;

        let settings = Settings::new();
        let contribution_value_degree = settings.ccv_degree;

        let result = calculate_contribution_value(
            contribution_value,
            contribution_value_degree,
            &mut numbers);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_calculate_dpt() {
        let result = calculate_dpt(
            10.0,
            10.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calculate_dpt_1() {
        let result = calculate_dpt(
            20.0,
            10.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 2.0);
    }

    #[test]
    fn calculate_dpt_2() {
        let result = calculate_dpt(
            1.0,
            10.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 0.09999999999999999);
    }

    #[test]
    fn calculate_dpt_3() {
        let result = calculate_dpt(
            1.0,
            1.0,
            1.5,
            1.0,
        );
        assert_eq!(result, 1.5);
    }

    #[test]
    fn calculate_dpt_in_loop() {
        let mut result = 0.0;
        for _n in 1..100 {
            result += calculate_dpt(
                1.0,
                10.0,
                1.0,
                100.0);
        }
        assert_eq!(result, 9.89999999999998);
        result = calculate_dpt(
            100.0,
            10.0,
            1.0,
            100.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_calculate_dpt_bonus() {
        let mut result = calculate_dpt_bonus(
            1);
        assert_eq!(result, 1.5);

        result = calculate_dpt_bonus(
            40);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_calculate_dpt_bonus_by_period() {
        let mut result = calculate_dpt_bonus_by_period(
            1);
        assert_eq!(result, 1.5);

        result = calculate_dpt_bonus_by_period(
            40 * 12);
        assert_eq!(result, 1.0);
    }
}