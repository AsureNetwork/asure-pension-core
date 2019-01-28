//macro_rules! log {
//    ($msg:expr) => {{
//        let state: i32 = get_log_state();
//        if state > 0 {
//            println!("log({}): {}", state, $msg);
//        }
//    }};
//}

pub mod calculations {
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

        println!("ref_value {},ccv {} = diff {} %", ref_value, ccv, diff);
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

    pub fn calculate_entitlement_months(periods: u64) -> f64 {
        return (periods as f64).powf(2.0) / 480f64;
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
}

#[cfg(test)]
mod tests {
    use crate::common::*;

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
        let result = calculations::avg(&numbers);
        assert_eq!(result, 2.0);
    }

//    #[test]
//    fn median() {
//        let mut numbers = [1.0, 0.1, 5.0];
//        let result = calculations::median(&mut numbers);
//
//        assert_eq!(result, 1.0);
//    }

    #[test]
    fn calculate_contribution_value() {
        let mut numbers = [1.0, 1.0, 1.0];

        let contribution_value = 1.0;
        let contribution_value_degree = 10.0;

        let result = calculations::calculate_contribution_value(
            contribution_value,
            contribution_value_degree,
            &mut numbers);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn calculate_dpt() {
        let result = calculations::calculate_dpt(
            10.0,
            10.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calculate_dpt_1() {
        let result = calculations::calculate_dpt(
            20.0,
            10.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 2.0);
    }

    #[test]
    fn calculate_dpt_2() {
        let result = calculations::calculate_dpt(
            1.0,
            10.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 0.09999999999999999);
    }

    #[test]
    fn calculate_dpt_3() {
        let result = calculations::calculate_dpt(
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
            result += calculations::calculate_dpt(
                1.0,
                10.0,
                1.0,
                100.0);
        }
        assert_eq!(result, 9.89999999999998);
        result = calculations::calculate_dpt(
            100.0,
            10.0,
            1.0,
            100.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn calculate_dpt_bonus() {
        let mut result = calculations::calculate_dpt_bonus(
            1);
        assert_eq!(result, 1.5);

        result = calculations::calculate_dpt_bonus(
            40);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calculate_dpt_bonus_by_period() {
        let mut result = calculations::calculate_dpt_bonus_by_period(
            1);
        assert_eq!(result, 1.5);

        result = calculations::calculate_dpt_bonus_by_period(
            40 * 12);
        assert_eq!(result, 1.0);
    }
}