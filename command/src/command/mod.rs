mod cut;
mod copy;
mod paste;

pub use cut::CutCommand;
pub use paste::PasteCommand;
pub use copy::CopyCommand;

pub trait Command {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
    fn undo(&mut self, app: &mut cursive::Cursive);
}