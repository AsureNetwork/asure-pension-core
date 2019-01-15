pub struct Settings {
    pub period: u64,
    pub current_avg_points: f64,
    pub current_points_degree: f64,
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
            current_points_degree: 10.0,
            current_contribution_value: 10.0,
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
//
//    pub fn avg() -> f64 {
//        let sum: f64 = 21.0;
//        let len: usize = 3;
//        sum / len as f64
//    }


    pub fn calculate_points(current_contribution_value: f64,
                            current_avg_points: f64,
                            amount: f64,
                            min: f64,
                            max: f64) -> f64 {
        let ccv = current_contribution_value;

        if amount > ccv {
            return (1f64 + (amount - ccv) / (max - ccv)) * current_avg_points;
        }
        if amount < ccv && min != amount {
            return ((amount - min) / (ccv - min)) * current_avg_points;
        } else if amount < ccv && min == amount {
            let minx = min - MIN_POSITIVE;
            return ((amount - minx) / (ccv - minx)) * current_avg_points;
        }
        1f64 //amount == ccv
    }

    pub fn calculate_avg_points_factor_by_period(index: u64) -> f64 {
        assert_ne!(index, 0);
        if index >= 40 * 12 {
            return 1.0;
        }
        let year = index % 12;
        calculate_avg_points_factor(year)
    }

    pub fn calculate_avg_points_factor(year: u64) -> f64 {
        assert_ne!(year, 0);
        if year >= 40 {
            return 1.0;
        }
        let y = year as f64;
        //[1,5..1.0] in 40 years
        //1.0+(40+1)^2/40/40*0,5
        let result = 1.0 + (((40.0 + 1.0 - y) * (40.0 + 1.0 - y)) / 40.0) / 40.0 * 0.5;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::common::*;

    #[test]
    fn calculate_points() {
        //let sumres = calculations::avg();
        //assert_eq!(sumres, 7.0);

        let mut result = calculations::calculate_points(
            10.0,
            1.0,
            10.0,
            10.0,
            20.0);
        assert_eq!(result, 1.0);

        result = calculations::calculate_points(
            10.0,
            1.0,
            20.0,
            10.0,
            20.0);
        assert_eq!(result, 2.0);

        result = calculations::calculate_points(
            10.0,
            1.0,
            1.0,
            1.0,
            20.0);
        assert_eq!(result, 0.000000000000000012335811384723961);
    }

    #[test]
    fn calculate_avg_points_factor() {
        let mut result = calculations::calculate_avg_points_factor(
            1);
        assert_eq!(result, 1.5);

        result = calculations::calculate_avg_points_factor(
            40);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn calculate_avg_points_factor_by_period() {
        let mut result = calculations::calculate_avg_points_factor_by_period(
            1);
        assert_eq!(result, 1.5);

        result = calculations::calculate_avg_points_factor_by_period(
            40 * 12);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn common_new_works() {
        let settings = Settings::new();
        assert_eq!(settings.eth, 0);
    }
}