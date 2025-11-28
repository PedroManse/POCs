use eg_cfg::CurrentTerminal;

fn main() {
    print1::<CurrentTerminal>(300);
    print2(300);
    #[cfg(feature = "l200")]
    {
        CurrentTerminal::extra_functionality();
    };
}

fn print1<T: eg_cfg::Terminal>(cost: usize) {
    T::show_text(&format!("This purchase will cost ${cost:.02}"));
}

fn print2(cost: usize) {
    print1::<CurrentTerminal>(cost);
}
