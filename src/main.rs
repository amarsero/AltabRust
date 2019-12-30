extern crate gtk;

use gtk::prelude::*;
mod altab;
use altab::Altab;
use std::sync::{mpsc, RwLock};
use crate::altab::entries::BaseEntry;

fn main() {
    if gtk::init().is_err() {
        println!("Failed  to initialize GTK.");
        return;
    }

    let main_src = include_str!("main.glade");
    let builder = gtk::Builder::new_from_string(main_src);

    let search_entry: gtk::SearchEntry = builder.get_object("searchEntry1").unwrap();

    let (altab, rx) = Altab::new();

    let altab = RwLock::new(altab);

    let rx = std::sync::Arc::new(rx);

    let list_store: gtk::ListStore = builder.get_object("liststore1").unwrap();

    let lsclone = list_store.clone();
    timeout_add(150, move || {
        return entry_recv_loop(rx.clone(), lsclone.clone());
    });

    let lsclone = list_store.clone();
    search_entry.connect_search_changed(move |search_entry| {
        search_changed(search_entry, altab.write().unwrap(), lsclone.clone());
    });

    let window: gtk::Window = builder.get_object("window1").unwrap();

    window.show_all();

    window.connect_destroy(|_| {
        gtk::main_quit();
    });
    gtk::main();
}

fn search_changed(search_entry: &gtk::SearchEntry, mut altab: std::sync::RwLockWriteGuard<Altab>, list_store: gtk::ListStore) {
    altab.stop_search();
    list_store.clear();
    altab.search_all(search_entry.get_text().unwrap().as_str().to_string());
}

fn entry_recv_loop(rx: std::sync::Arc<mpsc::Receiver<crate::altab::entries::ResultEntry>>, list_store: gtk::ListStore) -> gtk::prelude::Continue {
    while let Ok(entry) = rx.try_recv() {
        let tree_iter = list_store.append();
        let name = entry.1.name().to_value();
        list_store.set_value(&tree_iter, 1, &name);
    }
    return gtk::prelude::Continue(true);
}

