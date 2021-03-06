
extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("GraphApp.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").unwrap();    

    window.show_all();

    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    gtk::main();
}
