use crate::altab::entries::entry::BaseEntry;
use crate::altab::entries::shortcut_entry::ShortcutEntry;
use crate::altab::entries::ResultEntry;
use crate::altab::spell_checker;
use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};



pub struct Deposit {
    pub entries: Arc<RwLock<Vec<Arc<ShortcutEntry>>>>,
    pub total_run_count: i64,
}

type SendResult = Result<(), mpsc::SendError<ResultEntry>>;

impl Deposit {
    pub fn new() -> Deposit {
        return Deposit {
            entries: Arc::new(RwLock::new(Vec::new())),
            total_run_count: 0,
        };
    }

    pub fn remove_duplicates(&self) {
        let mut to_remove: BTreeSet<usize> = BTreeSet::new();
        {
            let reading = self.entries.read().unwrap();
            for i in 0..reading.len() {
                for j in (i + 1..reading.len()).rev() {
                    if reading[i].full_path == reading[j].full_path /*&& reading[j].full_path != null*/ ||
                    reading[i].name == reading[j].name
                    {
                        to_remove.insert(j);
                    }
                }
            }
        }
        if to_remove.len() > 0 {
            let mut writing = self.entries.write().unwrap();
            for index in to_remove.into_iter().rev() {
                writing.remove(index);
            }
        }
    }

    pub fn do_search(&self, search: &str, tx: mpsc::Sender<ResultEntry>, running: Arc<RwLock<bool>>) {
        if search == "" {
            let read = self.entries.read().unwrap();
            for i in 0..read.len() {
                if !*running.read().unwrap() 
                {
                    return;
                }
                let result = tx.send(ResultEntry(0.0, read[i].clone()));
                if result.is_err() {
                    return;
                }
            }
            return;
        }
        let mut state = SearchState {
            sent_count: 0,
            tx: tx,
            list: Box::new(Vec::new()),
            running: running
        };
        let upper = search.to_uppercase();
        let search_split: Vec<&str> = upper.split_whitespace().collect();
        let entries = self.entries.deref().read().unwrap().clone();
        for entry in entries.iter() {
            let mut added: bool = false;
            for j in entry.deref().name.to_uppercase().split_whitespace() {
                for k in &search_split {
                    if &j == k {
                        added = true;
                        state
                            .send_entry(ResultEntry(1.0 + entry.run_count as f32, entry.clone()))
                            .unwrap();
                        break;
                    }
                    if added {
                        break;
                    }
                }
                if !added && entry.deref().matches(search) {
                    state
                        .send_entry(ResultEntry(
                            1.0 + entry.run_count as f32 / 2.0,
                            entry.clone(),
                        ))
                        .unwrap();
                }
            }
        }

        if state.sent_count < 5 {
            let mut to_add = false;
            let mut list_of_edits: Vec<Vec<String>> = vec![Vec::new(); search_split.len()];
            for k in 0..search_split.len() {
                if search_split[k].len() < 4 {
                    continue;
                }
                list_of_edits[k] = spell_checker::word_edits(search_split[k]);

                for entry in entries.iter() {
                    for entry_word in entry.deref().name.to_uppercase().split_whitespace() {
                        if entry_word.len() < 4 {
                            continue;
                        }
                        for edited_word in list_of_edits[k].iter() {
                            if entry_word == edited_word {
                                to_add = true;
                                break;
                            }
                        }
                        if to_add {
                            state
                                .send_entry(ResultEntry(
                                    1.0 + entry.run_count as f32 / 4.0,
                                    entry.clone(),
                                ))
                                .unwrap();
                            to_add = false;
                            break;
                        }
                    }
                }
            }

            if state.sent_count < 3 && search.len() < 11 {
                to_add = false;
                for k in list_of_edits.into_iter() {
                    let edited = spell_checker::add_new_edits(k);
                    for entry in entries.iter() {
                        for entry_word in entry.deref().name.to_uppercase().split_whitespace() {
                            if entry_word.len() < 4 {
                                continue;
                            }
                            for edited_word in edited.iter() {
                                if entry_word == edited_word {
                                    to_add = true;
                                    break;
                                }
                            }
                            if to_add {
                                state
                                    .send_entry(ResultEntry(
                                        1.0 + entry.run_count as f32 / 8.0,
                                        entry.clone(),
                                    ))
                                    .unwrap();
                                to_add = false;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

struct SearchState {
    pub tx: mpsc::Sender<ResultEntry>,
    pub list: Box<Vec<ResultEntry>>,
    pub sent_count: u32,
    pub running: Arc<RwLock<bool>>
}

impl SearchState {
    fn send_entry(&mut self, entry: ResultEntry) -> SendResult {
        if !*self.running.read().unwrap() 
        {
            panic!("Search thread's done running");
        }
        self.tx.send(entry)?;
        self.sent_count += 1;
        return Ok(());
    }
}
