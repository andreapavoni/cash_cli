pub mod export;
pub mod import;
pub mod list;
pub mod record;
pub mod report;

pub trait Command {
    fn run(&self) -> ();
}
