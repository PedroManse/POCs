pub struct TerminalL300;

impl super::Terminal for TerminalL300 {
    fn show_text(s: &str) {
        println!("[L300]: {s}");
    }
}
