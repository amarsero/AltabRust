use erased_serde::Serialize;

pub trait BaseEntry: Serialize {
    fn run(&self) -> bool;
    fn matches(&self, search: &str) -> bool;
    fn name(&self) -> &str;
    fn icon(&self) -> i8;
    fn run_count(&self) -> i32;
}