//cargo run --example sim1
use asure_pension_core::*;
use asure_pension_core::common::Settings;

fn main() {
    println!("Pension Sim1");

    let mut settings = Settings::new();
    settings.eth = 10;


    let mut pension = Pension::new();
    pension.create_users(10);

    let payment_months = 40 * 12; // 40 years
    let generations = 8; // number of generations that pay into the systems
    let total_months = payment_months * generations;

    for i in 0..total_months {
        // until the nth (2) generation
        if i < 2 * payment_months {
            // add n (2) new users every year
            if i % 12 == 0 {
                pension.create_users(2);
            }

            // retire user every two years
            if i % 24 == 0 {
                let mut counter = 0;
                for user in &mut pension.users {
                    if user.activate_retirement() {
                        counter = counter + 1;
                    }

                    if counter == 1 {
                        break;
                    }
                }
            }
        }

        pension.start();
        let mut sum: f64 = 0.0;
        for user in &mut pension.users {
            user.pay(pension.current_period, 20.0);
            sum += 20.0;
        }
        pension.add_amount(sum);
        pension.payout();
        pension.end();

        for user in &pension.users
            {
                //Console.WriteLine(

                //    string.Format("{0,10}",user.Name)+" "+
                //    string.Format("{0,20}", user.Total)+ "  :  "+

                //    (user.PensionPaymentMonths+ " = "+user.PensionRecivedMonths+" / "+user.PensionReciveMonths));
                println!("User: {} - {} {} DPT: {}", user.id, user.wallet.eth, user.wallet.pension_eth, user.wallet.dpt.amount);
//            Console.WriteLine(
//                user.Name + "," +
//                    user.Total + "," +
//                    (user.PensionPaymentMonths + "," + user.PensionRecivedMonths + "," + user.PensionReciveMonths));
            }
    }
}