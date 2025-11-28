pub struct TerminalMock;

impl super::Terminal for TerminalMock {
    fn show_text(s: &str) {
        println!("[TERMINAL MOCK]: {s}")
    }
}
