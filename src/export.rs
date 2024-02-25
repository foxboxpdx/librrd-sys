use crate::{RRDCommand, convert_to_mutmutchar, time_t};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "export";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String> // for options with required args
}

impl RRDCommand for Command {
    fn execute(&self) -> bool { true }
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

    pub fn start(&mut self) {}
    pub fn end(&mut self) {}
    pub fn maxrows(&mut self) {}
    pub fn step(&mut self) {}
    pub fn enumds(&mut self) {}
    pub fn json(&mut self) {}
    pub fn showtime(&mut self) {}
    pub fn daemon(&mut self) {}
}

// Options
// start, end, maxrows, step, enumds, json, showtime, daemon
pub fn export(argc: i32, argv: Vec<String>, unused: i32, start: time_t, end: time_t, step: u64, col_count: u64, legend: Vec<Vec<String>>, data: Vec<f64>) -> bool {
    true
}