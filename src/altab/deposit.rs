use crate::altab::entries::shortcut_entry::ShortcutEntry;
use crate::altab::entries::ResultEntry;
use std::collections::BTreeSet;
use std::sync::{mpsc, RwLock};

type Entry = ShortcutEntry;

pub struct Deposit {
    pub entries: RwLock<Vec<Entry>>,
    pub total_run_count: i64,
}

impl Deposit {
    pub fn new() -> Deposit {
        return Deposit {
            entries: RwLock::new(Vec::new()),
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
        if !to_remove.is_empty() {
            let mut writing = self.entries.write().unwrap();
            for index in to_remove.into_iter().rev() {
                writing.remove(index);
            }
        }
    }

    pub fn do_search(&self, rx: mpsc::Receiver<String>, tx: mpsc::Sender<ResultEntry>) {
        let mut query = rx.recv().unwrap();
        loop {
            let read = self.entries.read().unwrap();
            let mut i = 0;
            query.make_ascii_lowercase();
            while i < read.len() {
                let score = search(&query, read[i].name.clone());
                if score > 50 {
                    tx.send(ResultEntry::new(score, &read[i])).unwrap();
                }
                i += 1;
                if let Some(new_query) = rx.try_iter().last() {
                    query = new_query;
                    i = 0;
                }
            }
            query = rx.recv().unwrap();
        }
    }
}

fn search(query: &str, mut name: String) -> u32 {
    debug_assert_eq!(query, query.to_lowercase(), "Input should be lowercase!");
    if query.is_empty() {
        return 0;
    }
    name.make_ascii_lowercase();
    if name.starts_with(&query) {
        return 100;
    }
    let mut max = 0;
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

fn word_score(qword: &str, nword: &str) -> u32 {
    debug_assert_eq!(qword, qword.to_lowercase(), "Input should be lowercase!");
    debug_assert_eq!(nword, nword.to_lowercase(), "Input should be lowercase!");
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
    match error {
        0 => 90,
        1 => {
            if qword.len() < 2 {
                0
            } else {
                80
            }
        }
        2 => {
            if qword.len() < 3 {
                0
            } else {
                70
            }
        }

        _ => {
            let total = (100 - error / qword.len() * 100) as u32;
            if total > 70 {
                69
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
        assert_eq!(
            super::search("hola", "hola".to_string()),
            100,
            "Exact match"
        );
        assert_eq!(
            super::search("hola", "carlos hola".to_string()),
            90,
            "Match on not first word"
        );
        assert_eq!(super::search("hola", "perro".to_string()), 0, "No Match");
        assert_eq!(super::search("", "hola".to_string()), 0, "Empty string");

        //deletes
        assert_eq!(super::search("gminola", "gominola".to_string()), 80);
        assert_eq!(super::search("gmnola", "gominola".to_string()), 70);
        assert!(super::search("gmnla", "gominola".to_string()) < 70);

        //transposes
        assert_eq!(super::search("ohla", "hola".to_string()), 80);
        assert_eq!(super::search("ohal", "hola".to_string()), 70);
        assert!(super::search("minuto", "imunot".to_string()) < 70);

        //replaces
        assert_eq!(super::search("bola", "hola".to_string()), 80);
        assert_eq!(super::search("bota", "hola".to_string()), 70);
        assert!(super::search("bote", "hola".to_string()) < 70);

        //inserts
        assert_eq!(super::search("hhola", "hola".to_string()), 80);
        assert_eq!(super::search("hhoola", "hola".to_string()), 70);
        assert!(super::search("hhoolla", "hola".to_string()) < 70);

        //short strings
        assert!(super::search("a", "oooooooo".to_string()) < 70);
        assert!(super::search("aa", "ooooooooo".to_string()) < 70);
        assert!(super::search("aaa", "ooooooooooo".to_string()) < 70);
        assert!(super::search("aaaa", "ooooooooooo".to_string()) < 70);
    }
}
