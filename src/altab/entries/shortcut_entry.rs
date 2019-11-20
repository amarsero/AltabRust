use crate::altab::entries::Entry;
use std::path::PathBuf;

pub struct ShortcutEntry {
    pub name: String,
    pub icon: i8,
    pub run_count: i32,
    pub full_path: PathBuf,
}

impl ShortcutEntry {
    pub fn new() -> ShortcutEntry {
        ShortcutEntry {
            name: String::new(),
            full_path: PathBuf::new(),
            run_count: 0,
            icon: 0,
        }
    }
}

impl Entry for ShortcutEntry {
    fn run(&self) -> bool {
        return false;
    }
    fn matches(&self, search: &str) -> bool {
        return false;
    }
    fn name(&self) -> &str {
        return &self.name;
    }
    fn icon(&self) -> i8 {
        return self.icon;
    }
    fn run_count(&self) -> i32 {
        return self.run_count;
    }
}
