use crate::{RRDCommand, convert_to_mutmutchar};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "update";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String> // for options with required args
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {true}
}
pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(filename: String) -> Builder {
        let data = Command { filename, ..Default::default()};
        Builder { data }
    }
    
    pub fn build(&mut self) -> Command {
        self.data.clone()
    }

    pub fn template(&mut self) {

    }

    pub fn daemon(&mut self) {

    }

    pub fn skip_past_updates(&mut self) {

    }

    pub fn locking(&mut self) {

    }
}
// Options:
// template, daemon, skip-past-updates, locking
pub fn update(argc: i32, argv: Vec<String>) -> bool {
    true
}
