use crate::{RRDCommand, convert_to_mutmutchar, rrd_restore};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "restore";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub xmlfile: String,
    pub rrdfile: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String> // for options with required args
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(xmlfile: String, rrdfile: String) -> Builder {
        let data = Command { xmlfile, rrdfile, ..Default::default()};
        Builder { data }
    }
    
    // xmlfile first, then rrdfile, then opts
    pub fn build(self) -> Command {
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        retval.argv.push(retval.xmlfile.clone());
        retval.argv.push(retval.rrdfile.clone());
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

    // flag
    pub fn range_check(mut self) -> Builder {
        self.data.flags.push("--range-check".to_string());
        self
    }

    // flag
    pub fn force_overwrite(mut self) -> Builder {
        self.data.flags.push("--force-overwrite".to_string());
        self
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_restore(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}