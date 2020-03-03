use crate::altab::deposit::Deposit;
use crate::altab::entries::shortcut_entry::ShortcutEntry;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use gio::{FileExt, LoadableIconExt};

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
    let mut vec: Vec<Arc<ShortcutEntry>> = Vec::new();

    for file_path in paths.unwrap() {
        let path = file_path.unwrap().path();
        if path.is_file() {
            match path.extension().map_or("", |v| v.to_str().unwrap()){
                "lnk" | "exe" => {
                    let entry = new_shorctut_entry(&path);
                    if deposit.entries.read().unwrap().as_slice().iter().any(|x| entry.name == (**x).name)
                    {
                        continue;
                    }
                    vec.push(Arc::new(entry));
                }
                _ => continue
            }
        }
    }

    if vec.len() > 0
    {
        deposit.entries.write().unwrap().append(&mut vec);
    }
}

fn new_shorctut_entry(full_path: &Path) -> ShortcutEntry {
    let mut entry: ShortcutEntry = ShortcutEntry::new();
    entry.full_path.push(full_path);
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

fn get_icon(path: &Path) -> Result<(), CrawlError> {
    return Err(CrawlError{});
    let file = gio::File::new_for_path(path);
    let file_info: gio::FileInfo = file.query_info("standar::*", gio::FileQueryInfoFlags::NONE, gio::NONE_CANCELLABLE)
                        .map_err(|_| CrawlError{})?;
    
    // let content_type = file_info.get_content_type()
    //         .ok_or(CrawlError{})?;
    
    // let app_info = gio::AppInfo::get_default_for_type(&content_type, false)
    //         .ok_or(CrawlError{})?;

    // let icon = file_info.get_icon()
    //         .ok_or(CrawlError{})?;

    let file_icon = gio::FileIcon::new(&file);

    // let (stream, _) = file_icon.load(20, gio::NONE_CANCELLABLE)
    //         .map_err(|_| CrawlError{})?;

    let pixbuf = gdk_pixbuf::Pixbuf::new_from_file(file_icon.to_string());
    
    if let Err(error) = pixbuf {
        println!("{}", file_icon.to_string());
        println!("{}", error);
    }
    

    //gtk::gio::File
    /*
    GError *error;
    GFile *file = g_file_new_for_path (argv[1]);
    GFileInfo *file_info = g_file_query_info (file,
                                              "standard::*",
                                              0,
                                              NULL,
                                              &error);

    const char *content_type = g_file_info_get_content_type (file_info);
    char *desc = g_content_type_get_description (content_type);
    GAppInfo *app_info = g_app_info_get_default_for_type (
                                  content_type,
                                  FALSE);

    /* you'd have to use g_loadable_icon_load to get the actual icon */
    GIcon *icon = g_file_info_get_icon (file_info);

    printf ("File: %s\nDescription: %s\nDefault Application: %s\n",
            argv[1],
            desc,
            g_app_info_get_executable (app_info));

    return 0;
    */

    return Ok(());
}
