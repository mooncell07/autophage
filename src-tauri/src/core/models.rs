use strum_macros::Display;

#[derive(Display, Debug)]
pub enum Signals {
    SessionStarting,
    SessionReady,
    BinaryAnalyzed,
    SessionClosed,
}
