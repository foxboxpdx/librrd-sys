use crate::{RRDCommand, convert_to_mutmutchar};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "tune";

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

    pub fn heartbear(&mut self) {}
    pub fn minimum(&mut self) {}
    pub fn maximum(&mut self) {}
    pub fn data_source_type(&mut self) {}
    pub fn data_source_rename(&mut self) {}
    pub fn deltapos(&mut self) {}
    pub fn deltaneg(&mut self) {}
    pub fn window_length(&mut self) {}
    pub fn failure_threshold(&mut self) {}
    pub fn alpha(&mut self) {}
    pub fn beta(&mut self) {}
    pub fn gamma(&mut self) {}
    pub fn gamma_deviation(&mut self) {}
    pub fn smoothing_window(&mut self) {}
    pub fn smoothing_window_deviation(&mut self) {}
    pub fn aberrant_reset(&mut self) {}
    pub fn step(&mut self) {}
    pub fn daemon(&mut self) {}
}

pub fn tune(argc: i32, argv: Vec<String>) -> bool {
    true
}