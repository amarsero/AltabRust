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
use ngrammatic::{Corpus, CorpusBuilder};
//use crate::altab::crawler::Crawler;

pub struct Altab {
    pub deposit: Arc<Deposit>,
    sender: mpsc::Sender<ResultEntry>,
    current_search_running: Option<Arc<RwLock<bool>>>,
    corpus: Arc<Corpus>
}

impl Altab {
    pub fn new() -> (Altab, mpsc::Receiver<ResultEntry>) {
        let (tx, rx) = mpsc::channel::<ResultEntry>();
        let mut altab = Altab {
            deposit: Arc::new(Deposit::new()),
            sender: tx,
            current_search_running: None,
            corpus: Arc::new(CorpusBuilder::new().arity(2).case_insensitive().finish())
        };
        (&mut altab).init();
        return (altab, rx);
    }
    fn init(&mut self) {
        let deposit = &*self.deposit.clone();
        crate::altab::persistence::load(deposit);
        crate::altab::crawler::crawl_new_path(deposit, &dirs::desktop_dir().unwrap());
        deposit.remove_duplicates();
        crate::altab::persistence::save(deposit);
        deposit.populate_corpus(&mut self.corpus);
    }

    pub fn stop_search(&mut self) {
        if let Some(search) = self.current_search_running.clone() {
            let mut signal = search.write().unwrap();
            *signal = false;
        }
        self.current_search_running = None;
    }

    pub fn search_all<'a>(&'a mut self, search: String) {        
        let depo = self.deposit.clone();
        let tx = self.sender.clone();
        let signal = Arc::new(RwLock::new(true));
        let clone = signal.clone();
        thread::spawn(move || {
            depo.do_search(&search, tx, clone);
        });
        self.current_search_running = Some(signal);
    }
}
