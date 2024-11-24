#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
mod document;
mod editor;
mod row;
mod terminal;

use editor::Editor;

pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
