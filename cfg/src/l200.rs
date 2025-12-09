pub struct TerminalL200;

impl super::Terminal for TerminalL200 {
    fn show_text(s: &str) {
        println!("[L200]: {s}");
    }
}

impl TerminalL200 {
    pub fn extra_functionality() {
        println!("[L200]: Something only the l200 can do!");
    }
}
