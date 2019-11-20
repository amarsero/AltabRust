use crate::altab::entries::Entry;

pub struct Deposit {
    pub entries: Vec<Box<dyn Entry>>,
    pub total_run_count: i64,
}

impl Deposit {
    pub fn new() -> Deposit {
        return Deposit {
            entries: Vec::new(),
            total_run_count: 0
        }
    }
}