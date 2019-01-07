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
    pension.create_users(5);

    for i in 0..24 {
        if i % 12 == 0 && i < 480 * 2 {
            pension.create_users(2);
        }
        if i % 24 == 0 && i < 480 * 2 {
            let mut counter = 0;
            for user in &pension.users {
                if pension.activate_retirement(&user) {
                    counter = counter + 1;
                }
                if counter == 1 {
                    break;
                }
            }
        }
        pension.pay();
        pension.payout();

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