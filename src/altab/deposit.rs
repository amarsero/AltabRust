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
type QueryRequest = (String, mpsc::Sender<ResultEntry>);

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

    pub fn do_search(
        &self,
        search: &str,
        tx: mpsc::Sender<ResultEntry>,
        running: Arc<RwLock<bool>>,
    ) {
        if search == "" {
            let read = self.entries.read().unwrap();
            for i in 0..read.len() {
                if !*running.read().unwrap() {
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
            tx,
            list: Box::new(Vec::new()),
            running,
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

        if state.sent_count < 5 && search.len() > 2 {
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
                        if !to_add {
                            for edited_word in list_of_edits[k].iter() {
                                if edited_word.contains(entry_word) {
                                    to_add = true;
                                    break;
                                }
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

            if state.sent_count < 3 && search.len() < 11 && search.len() > 2 {
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
                            if !to_add {
                                for edited_word in edited.iter() {
                                    if edited_word.contains(entry_word) {
                                        to_add = true;
                                        break;
                                    }
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
    pub running: Arc<RwLock<bool>>,
}

impl SearchState {
    fn send_entry(&mut self, entry: ResultEntry) -> SendResult {
        if !*self.running.read().unwrap() {
            panic!("Search thread's done running");
        }
        self.tx.send(entry)?;
        self.sent_count += 1;
        return Ok(());
    }
}

fn search(query: &str, name: &str) -> f32 {
    if name.starts_with(query) {
        return 1.0;
    }
    let mut max = 0.0;
    for qword in query.split_whitespace() {
        for nword in name.split_whitespace() {
            let score = word_score(qword, nword);
            if score > max {
                max = score;
            }
        }
    }
    return max;
}

fn word_score(qword: &str, nword: &str) -> f32 {
    debug_assert_eq!(qword, qword.to_lowercase(), "Input should be lowercase!");
    debug_assert_eq!(nword, nword.to_lowercase(), "Input should be lowercase!");
    if qword.is_empty() {
        return 0.0;
    }
    let mut error = 0;
    let mut qi = qword.chars().peekable();
    let mut ni = nword.chars().peekable();
    let mut q = qi.next();
    let mut n = ni.next();
    while q.is_some() && n.is_some() {
        if q.unwrap() != n.unwrap() {
            error += 1;
            let qnext = qi.peek();
            let nnext = ni.peek();
            
            if nnext.is_some()
                && qnext.is_some()
                && *nnext.unwrap() == q.unwrap()
                && *qnext.unwrap() == n.unwrap()
            {
                //transposes
                n = ni.next();
                q = qi.next();
            } else if qnext.is_some() && *qnext.unwrap() == n.unwrap() {
                //inserts
                q = qi.next();
            } else if nnext.is_some() && *nnext.unwrap() == q.unwrap() {
                //deletes
                n = ni.next();
            }
            //replaces is default error
        }
        q = q.and_then(|_| qi.next());
        n = n.and_then(|_| ni.next());
    }
    if qword.len() < 2 && error > 0 {
        return 0.0;
    }
    match error {
        0 => 0.9,
        1 => 0.8,
        2 => 0.7,
        _ => {
            let total = 1.0 - (error as f32 / qword.len() as f32);
            if total > 0.7 {
                0.69
            } else {
                total
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn search() {
        assert_eq!(super::search("hola", "hola"), 1.0, "Exact match");
        assert_eq!(super::search("hola", "carlos hola"), 0.9, "Match on not first word");
        assert_eq!(super::search("hola", "perro"), 0.0, "No Match");
        assert_eq!(super::search("HOla", "hoLa"), 1.0, "case insensitive");
        assert_eq!(super::search("", "hola"), 0.0, "Empty string");

        //deletes
        assert_eq!(super::search("ola", "hola"), 0.8);
        assert_eq!(super::search("la", "hola"), 0.7);
        assert!(super::search("a", "hola") < 0.7);

        //transposes
        assert_eq!(super::search("ohla", "hola"), 0.8);
        assert_eq!(super::search("ohal", "hola"), 0.7);
        assert!(super::search("minuto", "imunot") < 0.7);

        //replaces
        assert_eq!(super::search("bola", "hola"), 0.8);
        assert_eq!(super::search("bota", "hola"), 0.7);
        assert!(super::search("bote", "hola") < 0.7);

        //inserts
        assert_eq!(super::search("hhola", "hola"), 0.8);
        assert_eq!(super::search("hhoola", "hola"), 0.7);
        assert!(super::search("hhoolla", "hola") < 0.7);
    }
}
