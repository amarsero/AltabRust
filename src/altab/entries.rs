pub mod entry;
pub mod shortcut_entry;

use self::shortcut_entry::ShortcutEntry;
use std::marker::PhantomData;

pub use self::entry::*;



#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub struct ResultEntry {
    pub score: u32,
    pub name: String,
    pub image: PhantomData<u32>,
}

impl ResultEntry {
    pub fn new(score: u32, entry: &ShortcutEntry) -> Self {
        ResultEntry {
            name: entry.name.clone(),
            image: PhantomData,
            score,
        }
    }
}

// impl PartialOrd for ResultEntry {
    
// }

// impl Ord for ResultEntry {
//     fn clamp(self, min: Self, max: Self) -> Self
//     where
//         Self: Sized,
//     {
//         Ord::clamp(self, min.score, max.score)
//     }
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         Ord::cmp(&self.score, &other.score)
//     }
//     fn max(self, other: Self) -> Self
//     where
//         Self: Sized,
//     {
//         Ord::max(&self.score, &other.score)
//     }
//     fn min(self, other: Self) -> Self
//     where
//         Self: Sized,
//     {
//         Ord::min(&self.score, &other.score)
//     }
// }
