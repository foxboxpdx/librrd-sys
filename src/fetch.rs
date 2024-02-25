use crate::{convert_to_mutmutchar, rrd_fetch, convert_to_mutmutmutchar, RRDCommand, RRAType};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "fetch";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String>, // for options with required args
    pub cf: String // the RRA consolidation function (RRAType)
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
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        retval.argv.push(retval.filename.clone());
        // CF should come right after filename
        retval.argv.push(retval.cf.clone());
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
    pub fn resolution(mut self, var: &str) -> Builder {
        self.data.opts.insert("--resolution".to_string(), var.to_string());
        self
    }

    // req
    pub fn start(mut self, var: &str) -> Builder {
        self.data.opts.insert("--start".to_string(), var.to_string());
        self
    }

    // req
    pub fn end(mut self, var: &str) -> Builder {
        self.data.opts.insert("--end".to_string(), var.to_string());
        self
    }

    // flag
    pub fn align_start(mut self) -> Builder {
        self.data.flags.push("--align_start".to_string());
        self
    }

    // req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // Add a CF
    pub fn with_cf(mut self, var: RRAType) -> Builder {
        self.data.cf = var.to_string();
        self
    }

}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            let empty: Vec<Vec<String>> = Vec::new();
            let mut converted2 = convert_to_mutmutmutchar(empty);
            let mut rrd_values: Vec<*mut f64> = Vec::new();
            rrd_fetch(self.argc as c_int, 
                converted.as_mut_ptr(), 
                0i64 as *mut i64, 
                0i64 as *mut i64, 
                0u64 as *mut u64, 
                0u64 as *mut u64, 
                converted2.as_mut_ptr(),
                rrd_values.as_mut_ptr()) == 0
        }
    }
}
