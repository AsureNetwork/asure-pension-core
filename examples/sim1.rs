use asure_pension_core::common::Settings;
//cargo run --example sim1
use asure_pension_core::core::*;
use asure_pension_core::pension::*;

fn main() {
    println!("Hello, world!");
    run();
    let mut settings = Settings::new();
    settings.eth = 10;


    let mut pension = Pension::new();
    pension.create_users(10);

    let payment_months = 40 * 12; // 40 years
    let generations = 8; // number of generations that pay into the systems
    let total_months = payment_months * generations;

    for i in 0..total_months {
        if i % 12 == 0 && i < 480 * 2 {
            pension.create_users(2);
        }

        if i % 24 == 0 && i < 480 * 2 {
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

        pension.start();
        pension.pay();
        pension.payout();
        pension.end();

        for user in &pension.users
            {
                //Console.WriteLine(

                //    string.Format("{0,10}",user.Name)+" "+
                //    string.Format("{0,20}", user.Total)+ "  :  "+

                //    (user.PensionPaymentMonths+ " = "+user.PensionRecivedMonths+" / "+user.PensionReciveMonths));

                println!("{} - {}", user.name, user.total);
//            Console.WriteLine(
//                user.Name + "," +
//                    user.Total + "," +
//                    (user.PensionPaymentMonths + "," + user.PensionRecivedMonths + "," + user.PensionReciveMonths));
            }
    }
}