mod crawler;
mod deposit;
pub mod entries;
mod persistence;


use bincode::de;

use crate::altab::deposit::Deposit;
use crate::altab::entries::ResultEntry;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::thread;
extern crate dirs;
//use crate::altab::crawler::Crawler;

pub struct Altab {
    pub deposit: Arc<Deposit>,
    tx_query: mpsc::Sender<String>,
    rx_result: mpsc::Receiver<ResultEntry>,
}

impl Altab {
    pub fn new() -> Altab {        
        let (tx_query, rx_query) = mpsc::channel::<String>();
        let (tx_result, rx_result) = mpsc::channel::<ResultEntry>();
        let altab = Altab {
            deposit: Arc::new(Deposit::new()),
            tx_query,
            rx_result,
        };
        let dp = altab.deposit.clone();
        thread::spawn(move || {
            Altab::init(&*dp);
        });        
        let depo = altab.deposit.clone();
        thread::spawn(move || {
            depo.do_search(rx_query, tx_result);
        });
        return altab;
    }

    fn init(deposit: &Deposit) {
        crate::altab::persistence::load(deposit);
        crate::altab::crawler::crawl_new_path(deposit, &dirs::desktop_dir().unwrap());
        crate::altab::crawler::crawl_new_path(deposit, &dirs::home_dir().map(|mut x| {x.push("Desktop");x}).unwrap());
        crate::altab::crawler::crawl_new_path(deposit, &dirs::public_dir().map(|mut x| {x.push("Desktop");x}).unwrap());
        deposit.remove_duplicates();
        for entry in deposit.entries.read().unwrap().iter() {
            println!("{}", entry.name);
        }
        crate::altab::persistence::save(deposit);
    }

    pub fn search(&self, search: String)  ->  &mpsc::Receiver<ResultEntry>{
        self.tx_query.send(search).unwrap();
        self.rx_result.try_iter().count(); //Clean the previous results;
        &self.rx_result
    }  

    pub fn get_recv(&self)  ->  &mpsc::Receiver<ResultEntry>{
        &self.rx_result
    }
}
