mod crawler;
mod deposit;
mod entries;
mod persistence;

use crate::altab::deposit::Deposit;
use std::thread;
use std::sync::{Arc, mpsc};
use crate::altab::entries::ResultEntry;
extern crate dirs;
//use crate::altab::crawler::Crawler;

pub struct Altab {
    pub deposit: Arc<Deposit>,
}

impl Altab {
    pub fn new() -> Altab {
        let altab = Altab {
            deposit: Arc::new(Deposit::new()),
        };
        let dp = altab.deposit.clone();
        thread::spawn(move || {
            Altab::init(&*dp);
        });
        return altab;
    }
    fn init(deposit: &Deposit) {
        crate::altab::persistence::load(deposit);
        crate::altab::crawler::crawl_new_path(deposit, &dirs::desktop_dir().unwrap());
        deposit.remove_duplicates();
        crate::altab::persistence::save(deposit);
    }

    pub fn search_all<'a>(&'a self, search: String) -> mpsc::Receiver<ResultEntry> {
        let (tx, rx) = mpsc::channel();
        let depo = self.deposit.clone();
        thread::spawn(move || {
            depo.do_search(&search, tx);
        });
        return rx;
    }
}
