use crate::altab::deposit::Deposit;
use crate::altab::entries::shortcut_entry::ShortcutEntry;
use crate::altab::entries::*;
use std::fs;
use std::path::Path;

pub fn crawl_new_path(deposit: &mut Deposit, path: &str, size: i32) {
    let paths = fs::read_dir(path);
    if paths.is_err() {
        return;
    }

    for file_path in paths.unwrap() {
        let path = file_path.unwrap().path();
        if path.is_file() {
            match path.extension().map_or("", |v| v.to_str().unwrap()){
                "lnk" | "exe" => {
                    let entry = new_shorctut_entry(&path);
                    if deposit.entries.as_slice().iter().any(|x| entry.name == x.name())
                    {
                        continue;
                    }
                    deposit.entries.push(Box::new(entry));
                }
                _ => continue
            }
        }
    }
}

fn new_shorctut_entry(fullPath: &Path) -> ShortcutEntry {
    let mut entry: ShortcutEntry = ShortcutEntry::new();
    entry.full_path.push(fullPath);
    entry.name.push_str(entry.full_path.file_name().map_or("noname", |x| x.to_str().unwrap()));
    // if (info.Extension.ToLower() == ".lnk")
    // {
    //     IWshRuntimeLibrary.WshShell shell = new IWshRuntimeLibrary.WshShell(); //Create a new WshShell Interface
    //     IWshRuntimeLibrary.IWshShortcut link;
    //     link = (IWshRuntimeLibrary.IWshShortcut)shell.CreateShortcut(fullPath); //Link the interface to our shortcut
    //     string targetPath = link.TargetPath;
    //     targetPath = FindFile(targetPath);
    //     if (targetPath != null)
    //     {
    //         entry.Icon = Icon.ExtractAssociatedIcon(targetPath);
    //         entry.TargetPath = targetPath;
    //     }
    // }
    // else
    // {
    //     entry.TargetPath = fullPath;
    //     entry.Icon = Icon.ExtractAssociatedIcon(fullPath);
    // //}
    
    return entry;
}