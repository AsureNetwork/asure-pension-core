pub struct Settings {
    pub period: u64,
    pub current_avg_points: f64,
    pub current_points_degree: u128,
    pub current_contribution_value: f64,
    pub eth: u128,
    pub tokens: u128,
}


impl Settings {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Settings {
        Settings {
            period: 1,
            current_avg_points: 1.5,
            current_points_degree: 10,
            current_contribution_value: 10.0,
            eth: 0,
            tokens: 0,
        }
    }

    //fn is_odd(n: u32) -> bool {
    //  n % 2 == 1
    //}
}


pub mod calculations {
    pub fn avg() -> f64 {
        let sum: f64 = 21.0;
        let len: usize = 3;
        sum / len as f64
    }


    pub fn calculate_points(current_contribution_value: f64,
                            current_avg_points: f64,
                            amount: f64,
                            min: f64,
                            max: f64) -> f64 {
        let ccv = current_contribution_value;
        if amount > ccv {
            return (1f64 + (amount - ccv) / (max - ccv)) * current_avg_points;
        }
        if amount < ccv {
            return ((amount - min) / (ccv - min)) * current_avg_points;
        }
        1f64
    }
}

#[cfg(test)]
mod tests {
    use crate::common::*;

    #[test]
    fn calculate_points() {
        let sumres = calculations::avg();
        assert_eq!(sumres, 7.0);

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
        assert_eq!(result, 2.0);
    }

    #[test]
    fn common_new_works() {
        let settings = Settings::new();
        assert_eq!(settings.eth, 0);
    }
}