mod editor;
mod terminal;

use editor::Editor;
use terminal::*;

fn main() {
    Editor::default().run();
}
