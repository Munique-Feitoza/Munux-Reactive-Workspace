// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

pub mod shell;
pub mod parser;
pub mod filesystem;
pub mod monitor;
pub mod git;
pub mod ssh;
pub mod persistence;
pub mod completion;

pub use monitor::SystemMonitor;
