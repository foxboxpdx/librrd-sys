use crate::{convert_to_mutmutchar, rrd_last, rrd_lastupdate, RRDCommand};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "lastupdate";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String> // for options with required args
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(filename: String) -> Builder {
        let data = Command { filename, ..Default::default()};
        Builder { data }
    }
    
    pub fn build(self) -> Command {
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        retval.argv.push(retval.filename.clone());
        for f in &self.data.flags {
            retval.argv.push(f.to_string());
        }
        for (k, v) in &self.data.opts {
            retval.argv.push(k.to_string());
            retval.argv.push(v.to_string());
        }
        retval.argc = retval.argv.len() as i32;
        retval
    }

    // req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_lastupdate(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}
