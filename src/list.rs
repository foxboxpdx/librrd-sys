use crate::{convert_to_mutmutchar, rrd_list};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "list";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String>, // for options with required args
    pub path: String // rrdcached base_dir
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(path: String) -> Builder {
        let data = Command { path, ..Default::default()};
        Builder { data }
    }

    pub fn build(self) -> Command {
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        for f in &self.data.flags {
            retval.argv.push(f.to_string());
        }
        for (k, v) in &self.data.opts {
            retval.argv.push(k.to_string());
            retval.argv.push(v.to_string());
        }
        // Path comes at the end of args
        retval.argv.push(retval.path.clone());
        retval.argc = retval.argv.len() as i32;
        retval
    }

    // req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // flag
    pub fn noflush(mut self) -> Builder {
        self.data.flags.push("--noflush".to_string());
        self
    }

    // flag
    pub fn recursive(mut self) -> Builder {
        self.data.flags.push("--recursive".to_string());
        self
    }
}

impl Command {
    pub fn execute(&self) -> String {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            let retval = rrd_list(self.argc as c_int, converted.as_mut_ptr());
            match std::ffi::CString::from_raw(retval).into_string() {
                Ok(x) => x,
                Err(e) => { format!("Error converting CString: {:?}", e) }
            }
        }
    }
}
