pub mod entry;
pub mod shortcut_entry;

use self::shortcut_entry::ShortcutEntry;
use std::sync::Arc;

pub use self::entry::*;

#[derive(Clone)]
pub struct ResultEntry(pub f32, pub Arc<ShortcutEntry>);