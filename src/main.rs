use tui::app::{run, TuiInput};

fn main() {
    println!(
        "tui-standalone {} — launching TUI entrypoint",
        env!("CARGO_PKG_VERSION")
    );
    let mut input = TuiInput::default();
    input.url = std::env::args().nth(1).unwrap_or_default();
    let exit = run(input);
    println!(
        "tui exit: reason={:?} epilogue={:?}",
        exit.reason, exit.epilogue
    );
}
