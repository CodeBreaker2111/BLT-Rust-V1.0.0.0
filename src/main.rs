mod acceseories;
mod ui;
mod compile_file;

use crate::ui::main as userInterface;
use crate::compile_file::main as compileCode;

fn main() {
    let ui_results = userInterface();

    compileCode(ui_results.0, ui_results.1);
}