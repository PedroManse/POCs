pub trait Terminal {
    fn show_text(s: &str);
}

#[cfg(feature = "l300")]
mod l300;
#[cfg(feature = "l300")]
pub use l300::TerminalL300 as CurrentTerminal;

#[cfg(feature = "l200")]
mod l200;
#[cfg(feature = "l200")]
pub use l200::TerminalL200 as CurrentTerminal;

#[cfg(not(any(feature = "l200", feature = "l300")))]
mod mock;
#[cfg(not(any(feature = "l200", feature = "l300")))]
pub use mock::TerminalMock as CurrentTerminal;

#[cfg(test)]
mod test;
