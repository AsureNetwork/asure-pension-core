use std::error::Error;
use std::fs::File;
use serde::Serialize;
use crate::user::User;
use crate::Pension;
use std::path::Path;
use crate::user::PensionStatus;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvUser {
    period: u64,
    id: usize,
    wallet: f64,
    pension: f64,
    dpt: f64,
    last_dpt: f64,
}

impl CsvUser {
    pub fn new(period: u64, user: &User) -> Self {
        CsvUser {
            period,
            id: user.id,
            wallet: user.wallet.eth,
            pension: user.wallet.pension_eth,
            dpt: user.wallet.dpt.amount,
            last_dpt: user.last_dpt,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvPension {
    period: u64,
    total_eth: f64,
    total_dpt: f64,
    total_contributor: usize,
    total_pensioner: usize,
}

impl CsvPension {
    pub fn new(pension: &Pension) -> Self {
        let contributor_count = pension.users.iter().filter(|user| user.pension_status == PensionStatus::Run).count();
        let pensioner_count = pension.users.iter().filter(|user| user.pension_status == PensionStatus::Retirement).count();

        CsvPension {
            period: pension.current_period,
            total_eth: pension.total_eth,
            total_dpt: pension.total_dpt,
            total_contributor: contributor_count,
            total_pensioner: pensioner_count,
        }
    }
}

pub struct PensionCsvExporter {
    csv_users: Vec<CsvUser>,
    csv_pensions: Vec<CsvPension>,
}

impl PensionCsvExporter {
    pub fn new() -> Self {
        PensionCsvExporter {
            csv_users: Vec::new(),
            csv_pensions: Vec::new(),
        }
    }

    pub fn add_users(&mut self, pension: &Pension) {
        self.csv_users.append(&mut pension.users
            .iter()
            .map(|user|
                CsvUser::new(pension.current_period, user)
            )
            .collect()
        );

        ()
    }

    pub fn add_pension(&mut self, pension: &Pension) {
        self.csv_pensions.push(CsvPension::new(pension));

        ()
    }

    pub fn export_users<P>(&mut self, path: P) where P: AsRef<Path> {
        match export_csv(path, &self.csv_users) {
            Ok(()) => println!("user csv export erfolgreich"),
            Err(error) => eprintln!("{}", error),
        }
    }

    pub fn export_pensions<P>(&mut self, path: P) where P: AsRef<Path> {
        match export_csv(path, &self.csv_pensions) {
            Ok(()) => println!("pension csv export erfolgreich"),
            Err(error) => eprintln!("{}", error),
        }
    }
}

pub fn export_csv<P, T>(path: P, items: &[T]) -> Result<(), Box<Error>>
    where P: AsRef<Path>,
          T: Serialize
{
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);

    for user in items {
        wtr.serialize(user)?;
    }
    wtr.flush()?;

    Ok(())
}