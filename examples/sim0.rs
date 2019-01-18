use asure_pension_core::*;
use asure_pension_core::common::Settings;
use asure_pension_core::user::{User, PensionStatus};
use asure_pension_core::csvexport::{PensionCsvExporter};

fn main() {
    println!("Pension Simulation 0");

    let mut settings = Settings::new();
    settings.eth = 10;

    let mut pension = Pension::new();
    let mut pension_exporter = PensionCsvExporter::new();

    let payment_count = 40 * 12; // 40 years, every month
    let payment = 1.0;

    // all users pay 1 ETH every month
    // for the next 40 years
    pension.create_users(10);
    for _i in 0..payment_count {
        pension.start();

        let mut amount = 0.0;
        let contributors: Vec<&mut User> = pension.users.iter_mut().filter(|user| user.pension_status == PensionStatus::Run).collect();
        for user in contributors {
            match user.pay(pension.current_period, payment) {
                Ok(()) => amount += payment,
                Err(_) => {}
            }
        }
        pension.add_amount(amount);

        pension.payout();
        pension.end();

        print(&pension);
        pension_exporter.add_pension(&pension);
        pension_exporter.add_users(&pension);
    }

    // retire all current users
    pension.users.iter_mut().for_each(|user| {
        user.activate_retirement();
    });

    // now start a second generation which pays into the system
    // and pay out pensions
    pension.create_users(10);
    for _i in 0..payment_count {
        pension.start();

        let mut amount = 0.0;
        let contributors: Vec<&mut User> = pension.users.iter_mut().filter(|user| user.pension_status == PensionStatus::Run).collect();
        for user in contributors {
            match user.pay(pension.current_period, payment) {
                Ok(()) => amount += payment,
                Err(_) => {}
            }
        }
        pension.add_amount(amount);

        pension.payout();
        pension.end();

        print(&pension);
        pension_exporter.add_pension(&pension);
        pension_exporter.add_users(&pension);
    }

    pension_exporter.export_pensions("sim0-pensions.csv");
    pension_exporter.export_users("sim0-users.csv");
}

fn print(pension: &Pension) {
    let contributor_count = pension.users.iter().filter(|user| user.pension_status == PensionStatus::Run).count();
    let pensioner_count = pension.users.iter().filter(|user| user.pension_status == PensionStatus::Retirement).count();

    println!("Period: {}, Total Eth: {}, Total Contributor: {}, Total Pensioner: {}",
             pension.current_period, pension.total_eth, contributor_count, pensioner_count);
    for user in &pension.users {
        println!("User: {}, Wallet: {}, Pension: {}, DPT: {}",
                 user.id, user.wallet.eth, user.wallet.pension_eth, user.wallet.dpt.amount);
    }

    println!();
    println!("-------------------------");
    println!();
}