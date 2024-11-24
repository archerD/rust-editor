#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
// #![warn(clippy::restriction)]
// #![allow(
//     clippy::missing_docs_in_private_items,
//     clippy::implicit_return,
//     clippy::shadow_reuse,
//     clippy::print_stdout,
//     clippy::wildcard_enum_match_arm,
//     clippy::else_if_without_else)]
// #![allow(
//     clippy::absolute_paths,
//     clippy::question_mark_used,
//     clippy::single_call_fn,
//     clippy::string_slice,
//     clippy::std_instead_of_core,
//     clippy::exhaustive_structs,
//     clippy::pub_use)]
// // TODO: to be addressed at some point
// #![allow(clippy::pattern_type_mismatch, clippy::as_conversions, clippy::arithmetic_side_effects)]
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
