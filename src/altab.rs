mod deposit;
mod entries;
mod crawler;
use crate::altab::deposit::Deposit;
use std::path::PathBuf;
//use crate::altab::crawler::Crawler;

pub struct Altab {
    pub deposit: Deposit
}

impl Altab {
    pub fn new(startup_path: PathBuf) -> Altab {
        return Altab {
            deposit: Deposit::new(),
            

            }
    }
}