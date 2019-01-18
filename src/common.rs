pub struct Settings {
    pub period: u64,
    pub current_avg_points: f64,
    pub current_contribution_value_degree: f64,
    pub current_contribution_value: f64,
    pub eth: u128,
    pub tokens: f64,
}


impl Settings {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Settings {
        Settings {
            period: 1,
            current_avg_points: 1.5,
            current_contribution_value_degree: 10.0,
            current_contribution_value: 1.0,
            eth: 0,
            tokens: 0.0,
        }
    }

    //fn is_odd(n: u32) -> bool {
    //  n % 2 == 1
    //}
}


pub mod calculations {
    pub const MIN_POSITIVE: f64 = 0.0000000000000001;//std::f64::MIN_POSITIVE

    pub fn avg(numbers: &mut [f64]) -> f64 {
        //numbers.sort();
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = numbers.len() / 2;
        numbers[mid]
    }

    pub fn median(numbers: &mut [f64]) -> f64 {
        //numbers.sort();
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = numbers.len() / 2;
        numbers[mid]
    }

    pub fn calculate_contribution_value(contribution_value: f64, contribution_value_degree: f64, numbers: &[f64]) -> f64 {
        let mut nums = numbers.to_vec();
        let ref_value = self::median(&mut nums);
        let ccv = contribution_value;
        let diff = ((ref_value.max(ccv) - ref_value.min(ccv)) / ref_value.max(ccv)) * 100.0;

        println!("ref_value {},ccv {} = diff {} ", ref_value, ccv, diff);
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

    pub fn calculate_points(contribution_value: f64,
                            avg_points: f64,
                            amount: f64,
                            max: f64) -> f64 {
        let ccv = contribution_value;

        if amount > ccv {
            return (1.0 + (amount - ccv) / (max - ccv)) * avg_points;
        }
        if amount < ccv {
            return ((amount - MIN_POSITIVE) / (ccv - MIN_POSITIVE)) * avg_points;
        }
        //println!("HIER ist {}", 1f64 * avg_points);
        1f64 * avg_points //amount == ccv
    }

    pub fn calculate_avg_points_factor_by_period(index: u64) -> f64 {
        assert_ne!(index, 0);
        if index >= 40 * 12 {
            return 0.0;
        }
        let year = index % 12;
        calculate_avg_points_factor(year)
    }

    pub fn calculate_avg_points_factor(year: u64) -> f64 {
        assert_ne!(year, 0);
        if year >= 40 {
            return 0.0;
        }
        let y = year as f64;
        //[1,5..1.0] in 40 years
        //1.0+(40+1)^2/40/40*0,5
        let result = (((40.0 + 1.0 - y).powf(2.0)) / 1600f64) * 0.5;
        result
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
    fn median() {
        let mut numbers = [1.0, 0.1, 5.0];
        let result = calculations::median(&mut numbers);

        assert_eq!(result, 1.0);
    }

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
    fn calculate_points_0() {
        let result = calculations::calculate_points(
            10.0,
            1.0,
            10.0,
            20.0,
        );
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calculate_points_1() {
        let result = calculations::calculate_points(
            10.0,
            1.0,
            20.0,
            20.0,
        );
        assert_eq!(result, 2.0);
    }

    #[test]
    fn calculate_points_2() {
        let result = calculations::calculate_points(
            10.0,
            1.0,
            1.0,
            20.0,
        );
        assert_eq!(result, 0.09999999999999999);
    }

    #[test]
    fn calculate_points_3() {
        let result = calculations::calculate_points(
            1.0,
            1.5,
            1.0,
            1.0,
        );
        assert_eq!(result, 1.5);
    }

    #[test]
    fn calculate_points_in_loop() {
        let mut result = 0.0;
        for _n in 1..100 {
            result += calculations::calculate_points(
                10.0,
                1.0,
                1.0,
                100.0);
        }
        assert_eq!(result, 9.89999999999998);
        result = calculations::calculate_points(
            10.0,
            1.0,
            100.0,
            100.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn calculate_avg_points_factor() {
        let mut result = calculations::calculate_avg_points_factor(
            1);
        assert_eq!(result, 0.5);

        result = calculations::calculate_avg_points_factor(
            40);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn calculate_avg_points_factor_by_period() {
        let mut result = calculations::calculate_avg_points_factor_by_period(
            1);
        assert_eq!(result, 0.5);

        result = calculations::calculate_avg_points_factor_by_period(
            40 * 12);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn common_new_works() {
        let settings = Settings::new();
        assert_eq!(settings.eth, 0);
    }
}