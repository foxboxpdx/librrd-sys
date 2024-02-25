use crate::{convert_to_mutmutchar, rrd_dump, RRDCommand};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "dump";

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
    pub fn new(filename: &str) -> Builder {
        let data = Command { filename: filename.to_string(), ..Default::default()};
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
    pub fn daemon(mut self, address: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), address.to_string());
        self
    }

    // req
    pub fn header(mut self, h: &str) -> Builder {
        let header = match h {
            "none" => "none".to_string(),
            "xsd" => "xsd".to_string(),
            "dtd" => "dtd".to_string(),
            _ => panic!("Invalid header selection")
        };
        self.data.opts.insert("--header".to_string(), header);
        self
    }

    // flag
    pub fn no_header(mut self) -> Builder {
        self.data.flags.push("--no-header".to_string());
        self
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_dump(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}
