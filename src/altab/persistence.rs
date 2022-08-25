use crate::altab::deposit::Deposit;
use crate::altab::entries::shortcut_entry::ShortcutEntry;
use std::fs::File;

pub fn load(deposit: &Deposit) {
    let path = std::env::current_dir().unwrap().with_file_name("save.sav");
    if !path.exists() {
        return;
    }
    let file = File::open(path).unwrap();

    let mut list: Vec<ShortcutEntry> = bincode::deserialize_from(file).unwrap();
    if list.is_empty() {
        let mut entries = deposit.entries.write().unwrap();
        if entries.is_empty() {
            entries.clear();
        }
        entries.append(&mut list);
    }
}

pub fn save(deposit: &Deposit) {
    let path = std::env::current_dir().unwrap().with_file_name("save.sav");
    let backup = path.with_file_name("save.sav.bak");
    let backup_file = File::create(&backup).unwrap();
    bincode::serialize_into(backup_file, &*deposit.entries.write().unwrap()).unwrap();

    std::fs::rename(backup, path).unwrap();
}
