//! State tracking between the server and the checking routine

use crate::simple_time::SimpleTime;

/// Whether the window is open or closed
#[derive(Debug, PartialEq, Eq)]
pub enum WindowState {
    /// Opened blinds
    Opened,
    /// Closed blinds
    Closed,
}

/// The current state of scheduled opening and closing and if the window is currently opened or
/// closed
pub struct State {
    /// When we should start opening the blinds
    pub start: Option<SimpleTime>,
    /// When we should stop opening the blinds
    pub end: Option<SimpleTime>,
    /// What the current state of the blinds is
    pub current: WindowState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            current: WindowState::Closed,
        }
    }
}
