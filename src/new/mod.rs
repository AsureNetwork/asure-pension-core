use crate::new::types::*;
use crate::new::contributor::Contributor;
use crate::new::pensioner::Pensioner;
use crate::new::user::User;
use crate::new::pension::Pension;

pub mod contributor;
pub mod doneuser;
pub mod pension;
pub mod pensioner;
pub mod types;
pub mod user;

#[warn(unused_variables)]
pub trait PensionSimulation {
    fn name(&mut self) -> String;

    fn new_contributors(&mut self, period: Period) -> u64 {
        match period {
            1 => 10,
            _ => 0
        }
    }

    fn should_retire(&mut self, contributor: &Contributor, _period: Period) -> bool {
        contributor.contributions.len() == 480
    }

    fn should_contribute(&mut self, _contributor: &Contributor, _period: Period) -> Option<Unit> {
        Some(1.0)
    }

    fn should_claim_pension(&mut self, _pensioner: &Pensioner, _period: Period) -> bool {
        true
    }
}

pub fn simulate<T>(mut simulation: T) -> Result<(), String> where T: PensionSimulation {
    println!("Pension {}", simulation.name());

    let mut users: Vec<User> = vec![];
    let mut pension = Pension::new();
    //let mut pension_exporter = PensionCsvExporter::new();

    loop {
        pension.start_new_period();

        // 1. Create new contributors
        add_new_contributor(&mut simulation, &mut pension, &mut users);

        // 2. Retire all selected contributors so they become pensioners
        users = retire_contributor(&mut simulation, &mut pension, users);

        // 3. Let all selected contributors pay into the pension system
        contribute(&mut simulation, &mut pension, &mut users);


        // 4. Payout pensions to all selected pensioners
        claim_pensions(&mut simulation, &mut pension, &mut users);

        // 5. Calculate and distribute DPT based on their contribution
        //    of the current period
        claim_dpts(&mut simulation, &mut pension, &mut users)?;


        // 6. Remove all pensioners from the system who got their complete pension
        users = remove_done_pensioners(&mut pension, users);


        // 7. Log state after period is done
        print(&pension, &users);
        //pension_exporter.add_pension(&pension);
        //pension_exporter.add_users(&pension);

        // 8. Repeat until all users retired and got their complete pension
        // TODO: Implement PartialEq?
        if users.iter().all(|user| {
            match user.to_done_user() {
                Some(_) => true,
                None => false
            }
        }) {
            break;
        }
    }

    //println!("{:?}", pension);
    //
    //pension_exporter.export_pensions(format!("{}-pensions.csv", simulation.name().to_lowercase()));
    //pension_exporter.export_users(format!("{}-users.csv", simulation.name().to_lowercase()));
    Ok(())
}

fn add_new_contributor<T>(simulation: &mut T, pension: &mut Pension, users: &mut Vec<User>) where T: PensionSimulation {
    for _ in 0..simulation.new_contributors(pension.period) {
        let new_user = User::new();
        match &new_user {
            User::Contributor(contributor) => pension.join(contributor),
            _ => panic!("user is not a contributor")
        }
        users.push(new_user);
    }
}

fn retire_contributor<T>(simulation: &mut T, pension: &mut Pension, users: Vec<User>) -> Vec<User> where T: PensionSimulation {
    users.into_iter()
        .map(|user| {
            match user {
                User::Contributor(contributor) => {
                    if simulation.should_retire(&contributor, pension.period) {
                        pension.retire(contributor)
                    } else {
                        User::Contributor(contributor)
                    }
                }
                _ => user
            }
        })
        .collect()
}

fn remove_done_pensioners(pension: &mut Pension, users: Vec<User>) -> Vec<User> {
    users.into_iter()
        .map(|user| {
            match user {
                User::Pensioner(pensioner) => pension.try_finish(pensioner),
                _ => user
            }
        })
        .collect()
}

fn contribute<T>(simulation: &mut T, pension: &mut Pension, users: &mut Vec<User>) where T: PensionSimulation {
    users.iter_mut()
        .filter_map(|user| user.to_contributor_mut())
        .for_each(|contributor| {
            if let Some(contribution) = simulation.should_contribute(contributor, pension.period) {
                match pension.contribute(contributor, contribution) {
                    Err(err) => panic!(err),
                    _ => (),
                }
            }
        });
}

fn claim_pensions<T>(simulation: &mut T, pension: &mut Pension, users: &mut Vec<User>) where T: PensionSimulation {
    let period = pension.period;

    users.iter_mut()
        .filter_map(|user| user.to_pensioner_mut())
        .filter(|pensioner| simulation.should_claim_pension(pensioner, period))
        .for_each(|pensioner| {
            match pension.claim_pension(pensioner) {
                Err(err) => panic!(err),
                _ => (),
            }
        });
}

fn claim_dpts<T>(_simulation: &mut T, pension: &mut Pension, users: &mut Vec<User>) -> Result<(), String> where T: PensionSimulation {
    // TODO: Logically I think this should happen as step 4 and the name of
    //       the method should reflect what it does.
    pension.prepare_claim_dpt(users)?;
    users.iter_mut()
        .filter_map(|user| user.to_contributor_mut())
        .for_each(|contributor| {
            match pension.claim_dpt(contributor) {
                Err(err) => panic!(err),
                _ => (),
            }
        });

    Ok(())
}

pub fn print(pension: &Pension, users: &[User]) {
    let contributor_count = users
        .iter()
        .filter_map(|user| user.to_contributor())
        .count();

    let pensioner_count = users
        .iter()
        .filter_map(|user| user.to_pensioner())
        .count();

    let done_count = users
        .iter()
        .filter_map(|user| user.to_done_user())
        .count();


    let total_pension_eth = pension.pensions_total;

    println!("Period: {}, Total Eth: {}, Total Pension Eth: {}, Total DPT: {}, Total Contributor: {}, Total Pensioner: {}, Total Done: {}",
             pension.period, pension.contributions_total, pension.pensions_total, pension.dpt_total, contributor_count, pensioner_count, done_count);
    for user in users {
        match user {
            User::Contributor(contributor) => {
                let last_dpt = match contributor.dpts.get(&pension.period) {
                    Some(dpt) => format!("{}", dpt),
                    None => "0".to_string()
                };

                println!("User: {}, Status: {}, Wallet: {}, Pension: {}, Pension Months Allowed: {}, Pensions Months Received: {}, DPT: {} + ({})",
                         contributor.id(), "Contributor", contributor.wallet(), 0, contributor.allowed_pension_periods(),
                         0, contributor.dpt_total(), last_dpt);
            }
            User::Pensioner(pensioner) => {
                let contributor = &pensioner.contributor;
                println!("User: {}, Status: {}, Wallet: {}, Pension: {}, Pension Months Allowed: {}, Pensions Months Received: {}, DPT: {}",
                         contributor.id(), "Pensioner", contributor.wallet(), pensioner.total_pension(),
                         contributor.allowed_pension_periods(), pensioner.pension_periods(), contributor.dpt_total());
            }
            User::Done(done_user) => {
                let pensioner = &done_user.pensioner;
                let contributor = &pensioner.contributor;
                println!("User: {}, Status: {}, Wallet: {}, Pension: {}, Pension Months Allowed: {}, Pensions Months Received: {}, DPT: {}",
                         contributor.id(), "Done", contributor.wallet(), pensioner.total_pension(),
                         contributor.allowed_pension_periods(), pensioner.pension_periods(), contributor.dpt_total());
            }
        }
    }

    println!();
    println!("-------------------------");
    println!();
}