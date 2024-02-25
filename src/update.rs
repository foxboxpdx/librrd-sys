use crate::{RRDCommand, convert_to_mutmutchar, rrd_update};
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
    pub opts: HashMap<String, String>, // for options with required args
    pub updates: Vec<String>, // Data to insert into the RRD
    pub hyphens: bool // Whether to put '--' before the updates
                      // (required if any updates have a negative time value)
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
        if retval.hyphens {
            retval.argv.push("--".to_string());
        }
        retval.argv.append(&mut retval.updates);
        retval.argc = retval.argv.len() as i32;
        retval
    }

    // req
    pub fn template(mut self, var: &str) -> Builder {
        self.data.opts.insert("--template".to_string(), var.to_string());
        self
    }

    // req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // flag
    pub fn skip_past_updates(mut self) -> Builder {
        self.data.flags.push("--skip-past-updates".to_string());
        self
    }

    // The update data to push into the RRD
    // Accepts a bunch of different formats, check 'man rrdupdate'
    // for details and examples.
    pub fn with_update(mut self, update: &str) -> Builder {
        if update.starts_with('-') { self.data.hyphens = true; }
        self.data.updates.push(update.to_string());
        self
    }

    // There's an option in the C code for rrd_update() called
    // 'locking' that requires an argument.  I'm not sure what
    // it takes so I'm just leaving a placeholder here
    pub fn locking(self, _var: &str) -> Builder { self }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_update(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}
