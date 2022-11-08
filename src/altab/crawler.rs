use parselnk::Lnk;

use crate::altab::deposit::Deposit;
use crate::altab::entries::shortcut_entry::ShortcutEntry;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct CrawlError;

impl std::fmt::Display for CrawlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error crawling file") /* Marco is gai */
    }
}
impl std::error::Error for CrawlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub fn crawl_new_path(deposit: &Deposit, path: &Path) {
    // TODO: fix get_icon
    //get_icon(Path::new("/home/pWnd/Descargas/gitkraken-amd64.deb")).unwrap();
    let paths = fs::read_dir(path);
    if paths.is_err() {
        return;
    }
    let mut vec: Vec<ShortcutEntry> = Vec::new();

    for file_path in paths.unwrap() {
        let path = file_path.unwrap().path();
        if path.is_file() {
            match path.extension().map_or("", |v| v.to_str().unwrap()) {
                "lnk" | "exe" => {
                    let entry = new_shorctut_entry(&path);
                    if deposit
                        .entries
                        .read()
                        .unwrap()
                        .as_slice()
                        .iter()
                        .any(|x| entry.name == x.name)
                    {
                        continue;
                    }
                    vec.push(entry);
                }
                _ => continue,
            }
        }
    }

    if !vec.is_empty() {
        deposit.entries.write().unwrap().append(&mut vec);
    }
}

fn new_shorctut_entry(full_path: &Path) -> ShortcutEntry {
    let lnk_extension = OsStr::new("lnk");
    let mut entry: ShortcutEntry = ShortcutEntry::new();
    if full_path.extension() == Some(lnk_extension) {
        if let Ok(lnk) = Lnk::try_from(full_path) {
            println!("{}", full_path.to_str().unwrap_or("pipi"));
            let relative = lnk
                .relative_path()
                .unwrap_or_else(|| PathBuf::from_str("pipi").unwrap());
            println!("{}", relative.to_str().unwrap_or("pipi"));
            let joined = full_path
                .parent()
                .unwrap_or_else(|| Path::new("pipi"))
                .join(relative)
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from_str("pipi").unwrap());
            println!("{}", joined.to_str().unwrap_or("pipi"));
            // entry.full_path = PathBuf::from("\\\\?\\C:\\Users\\pWnd.-\\AppData\\Roaming\\Spotify\\Spotify.exe");
            println!("{}", entry.full_path.to_str().unwrap_or("pipi"));
            entry.full_path = lnk
                .relative_path()
                .and_then(|x| full_path.join(x).canonicalize().ok())
                .unwrap_or_else(|| full_path.to_path_buf());
            entry.name = lnk.string_data.name_string.unwrap_or_else(|| {
                entry
                    .full_path
                    .file_stem()
                    .and_then(|x| x.to_str())
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "noname".to_string())
            });
        }
    } else {
        entry.full_path.push(full_path);
        entry.name.push_str(
            entry
                .full_path
                .file_stem()
                .map_or("noname", |x| x.to_str().unwrap()),
        );
    }
    return entry;
}

#[allow(dead_code, unused_variables, unreachable_code)]
fn get_icon(path: &Path) -> Result<(), CrawlError> {
    return Err(CrawlError {});

    return Ok(());
}
