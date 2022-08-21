mod crawler;
mod deposit;
pub mod entries;
mod persistence;
mod spell_checker;

use crate::altab::deposit::Deposit;
use std::thread;
use std::sync::{Arc, mpsc, RwLock};
use crate::altab::entries::ResultEntry;
extern crate dirs;
//use crate::altab::crawler::Crawler;

pub struct Altab {
    pub deposit: Arc<Deposit>,
    current_search_running: Option<Arc<RwLock<bool>>>
}

impl Altab {
    pub fn new() -> Altab {
        let altab = Altab {
            deposit: Arc::new(Deposit::new()),
            current_search_running: None
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

    pub fn search<'a>(&'a mut self, search: String) -> mpsc::Receiver<ResultEntry>{        
        let depo = self.deposit.clone();
        let (tx, rx) = mpsc::channel::<ResultEntry>();
        let signal = Arc::new(RwLock::new(true));
        let clone = signal.clone();
        thread::spawn(move || {
            depo.do_search(&search, tx, clone);
        });
        if let Some(search) = self.current_search_running.clone() {
            let mut signal = search.write().unwrap();
            *signal = false;
        }
        self.current_search_running = Some(signal);
        return rx;
    }
}
