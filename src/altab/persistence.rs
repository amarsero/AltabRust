use crate::altab::deposit::Deposit;
use crate::altab::entries::shortcut_entry::ShortcutEntry;
use std::fs::File;
use std::sync::Arc;

pub fn load(deposit: &Deposit) {
    let path = std::env::current_dir().unwrap().with_file_name("save.sav");
    if !path.exists() {
        return;
    }
    let file = File::open(path).unwrap();

    let list: Vec<ShortcutEntry> = bincode::deserialize_from(file).unwrap();
    if list.len() > 0 {
        let mut arc_list: Vec<Arc<ShortcutEntry>> = list.into_iter().map(|x| Arc::new(x)).collect();
        let mut entries = deposit.entries.write().unwrap();
        if entries.len() > 0 {
            entries.clear();
        }
        entries.append(&mut arc_list);
    }
}

pub fn save(deposit: &Deposit) {
    let path = std::env::current_dir().unwrap().with_file_name("save.sav");
    let backup = path.with_file_name("save.sav.bak");
    let backup_file = File::create(&backup).unwrap();
    bincode::serialize_into(backup_file, &*deposit.entries.write().unwrap()).unwrap();

    std::fs::rename(backup, path).unwrap();
}
