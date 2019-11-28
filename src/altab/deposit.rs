use crate::altab::entries::shortcut_entry::ShortcutEntry;
use crate::altab::entries::entry::BaseEntry;
use crate::altab::entries::ResultEntry;
use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};

pub struct Deposit {
    pub entries: Arc<RwLock<Vec<Arc<ShortcutEntry>>>>,
    pub total_run_count: i64,
}

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

    pub fn do_search(&self, search: &str, tx: mpsc::Sender<ResultEntry>) {
        if search == "" {
            let read = self.entries.read().unwrap();
            for i in 0..read.len() {
                let result = tx.send(ResultEntry(0.0, read[i].clone()));
                if result.is_err() {
                    return;
                }
            }
        }
        let mut sentCount = 0;
        let upper = search.to_uppercase();
        let search_split: Vec<&str> = upper.split_whitespace().collect();
        let mut added: bool;
        let entries = self.entries.deref().read().unwrap().clone();
        for entry in entries.iter() {
            added = false;
            for j in entry.deref().name.to_uppercase().split_whitespace() {
                for k in &search_split {
                    if &j == k {
                        added = true;
                        let result = tx.send(ResultEntry(entry.run_count as f32, entry.clone()));
                        if result.is_err() {
                            return;
                        }
                        sentCount +=1;
                        break;
                    }
                    if added {
                        break;
                    }
                }
                if !added && entry.deref().matches(search) {
                    let result = tx.send(ResultEntry(entry.run_count as f32 / 2.0, entry.clone()));
                    if result.is_err() {
                        return;
                    }
                    sentCount +=1;
                }
            }
        }

        if sentCount < 5 {
            
        }
        /*

        if (list.Count < 5)
        {
            search = search.ToUpper();
            string[] split = search.Split(' ');
            List<string>[] listOfEdits = new List<string>[split.Length];
            added = false;
            for (int k = 0; k < split.Length; k++)
            {
                if (split[k].Length < 4) continue;
                listOfEdits[k] = SpellChecker.Edits(split[k]);

                for (int i = 0; i < Entries.Count; i++)
                {
                    foreach (var entry in Entries[i].Name.ToUpper().Split(' '))
                    {
                        if (entry.Length < 4) continue;
                        for (int j = 0; j < listOfEdits[k].Count; j++)
                        {
                            if (entry == listOfEdits[k][j])
                            {
                                added = true;
                                break;
                            }
                        }
                        if (added)
                        {
                            if (!list.ContainsValue(Entries[i]))
                            {
                                list.Add((Entries[i].RunCount + 1) / 4, Entries[i]);
                            }
                            added = false;
                            break;
                        }
                    }
                }
            }

            if (list.Count < 3 && search.Length < 11)
            {
                added = false;
                for (int k = 0; k < split.Length; k++)
                {
                    if (listOfEdits[k] == null) continue;
                    listOfEdits[k] = SpellChecker.Edits(listOfEdits[k]);
                    for (int i = 0; i < Entries.Count; i++)
                    {
                        foreach (var entry in Entries[i].Name.ToUpper().Split(' '))
                        {
                            if (entry.Length < 4) continue;
                            for (int j = 0; j < listOfEdits[k].Count; j++)
                            {
                                if (entry == listOfEdits[k][j])
                                {
                                    added = true;
                                    break;
                                }
                            }
                            if (added)
                            {
                                if (!list.ContainsValue(Entries[i]))
                                {
                                    list.Add((Entries[i].RunCount + 1) / 8, Entries[i]);
                                }
                                added = false;
                                break;
                            }
                        }
                    }
                }
            }
        }
        return list.ToList();*/
    }
}
