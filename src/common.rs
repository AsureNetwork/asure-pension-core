
pub struct Settings{
    pub period:u64,
    pub current_avg_points:f64,
    pub current_points_degree: u128,
    pub current_contribution_value: f64,
    pub eth:u128,
    pub tokens:u128
}


impl Settings {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Settings {
        Settings {
            period: 1,
            current_avg_points : 1.5,
            current_points_degree:10,
            current_contribution_value: 10.0,
            eth:0,
            tokens:0
        }
    }

    //fn is_odd(n: u32) -> bool {
    //  n % 2 == 1
    //}
}


#[cfg(test)]
mod tests {
    use crate::common::Settings;

    #[test]
    fn common_new_works() {
        let settings = Settings::new();
        assert_eq!(settings.eth, 0);
    }
}