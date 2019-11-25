use crate::altab::entries::shortcut_entry::ShortcutEntry;
use std::sync::RwLock;


pub struct Deposit {
    pub entries: RwLock<Vec<Box<ShortcutEntry>>>,
    pub total_run_count: i64,
}

impl Deposit {
    pub fn new() -> Deposit {
        return Deposit {
            entries: RwLock::new(Vec::new()),
            total_run_count: 0
        }
    }
}