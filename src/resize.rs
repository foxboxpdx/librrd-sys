use crate::{RRDCommand, convert_to_mutmutchar};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "resize";

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

    pub fn infilename(&mut self, name: String) {}
    pub fn target_rra(&mut self) {}
    pub fn grow(&mut self) {}
    pub fn shrink(&mut self) {}
    pub fn modify(&mut self) {}
}

// Doesn't use optparse_init
// argc must == 5
// argv[1] - infilename
// argv[2] - target_rra
// argv[3] - "GROW" or "SHRINK"
// argv[4] - modify (number of rows to grow or shrink)
pub fn resize(argc: i32, argv: Vec<String>) -> bool {
    true
}