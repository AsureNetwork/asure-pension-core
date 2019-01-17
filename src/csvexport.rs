use std::error::Error;
use std::fs::File;
use serde::Serialize;
use crate::user::User;
use crate::Pension;
use std::path::Path;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvUser {
    period: u64,
    id: usize,
    wallet: f64,
    pension: f64,
    dpt: f64,
}

impl CsvUser {
    pub fn new(period: u64, user: &User) -> Self {
        CsvUser {
            period,
            id: user.id,
            wallet: user.wallet.eth,
            pension: user.wallet.pension_eth,
            dpt: user.wallet.dpt.amount,
        }
    }
}

pub struct PensionCsvExporter {
    csv_users: Vec<CsvUser>,
}

impl PensionCsvExporter {
    pub fn new() -> Self {
        PensionCsvExporter {
            csv_users: Vec::new(),
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

    pub fn export<P>(&mut self, path: P) where P: AsRef<Path> {
        match export_csv(path, &self.csv_users) {
            Ok(()) => println!("user csv export erfolgreich"),
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