use crate::{convert_to_mutmutchar, convert_to_mutmutmutchar, rrd_xport, RRAType, RRDCommand};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "export";

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String>, // for options with required args
    pub defs: Vec<String>, // source definitions
    pub cdefs: Vec<String>, // cdef definitions
    pub xports: Vec<String> // xport definitions
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new() -> Builder {
        let data = Command { ..Default::default()};
        Builder { data }
    }
    
    pub fn build(&mut self) -> Command {
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        for f in &self.data.flags {
            retval.argv.push(f.to_string());
        }
        for (k, v) in &self.data.opts {
            retval.argv.push(k.to_string());
            retval.argv.push(v.to_string());
        }
        retval.argv.append(&mut retval.defs);
        retval.argv.append(&mut retval.cdefs);
        retval.argv.append(&mut retval.xports);
        retval.argc = retval.argv.len() as i32;
        retval
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

    // req
    pub fn maxrows(mut self, var: &str) -> Builder {
        self.data.opts.insert("--maxrows".to_string(), var.to_string());
        self
    }

    // req
    pub fn step(mut self, var: &str) -> Builder {
        self.data.opts.insert("--step".to_string(), var.to_string());
        self
    }

    // flag
    pub fn enumds(mut self) -> Builder {
        self.data.flags.push("--enumds".to_string());
        self
    }

    // flag
    pub fn json(mut self) -> Builder {
        self.data.flags.push("--json".to_string());
        self
    }

    // flag
    pub fn showtime(mut self) -> Builder {
        self.data.flags.push("--showtime".to_string());
        self
    }

    // req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // Add a DEF
    pub fn with_def(mut self, name: &str, rrd: &str, source: &str, rra: RRAType) -> Builder {
        let s = format!("DEF:{}={}:{}:{}", name, rrd, source, rra.to_string());
        self.data.defs.push(s);
        self
    }

    // Add a CDEF
    pub fn with_cdef(mut self, name: &str, exp: &str) -> Builder {
        let s = format!("CDEF:{}={}", name, exp);
        self.data.cdefs.push(s);
        self
    }

    // Add an XPORT
    pub fn with_xport(mut self, source: &str, legend: &str) -> Builder {
        let s = format!("XPORT:{}:{}", source, legend);
        self.data.xports.push(s);
        self
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            let empty: Vec<Vec<String>> = Vec::new();
            let mut converted2 = convert_to_mutmutmutchar(empty);
            // rrd_value_t is just an f64
            let mut otherempty: Vec<*mut f64> = Vec::new();
            let (a, b, c, d, e) = (0i32, 0i64, 0i64, 0u64, 0u64);
            rrd_xport(self.argc as c_int,
                converted.as_mut_ptr(), 
                a as *mut i32, 
                b as *mut i64, 
                c as *mut i64, 
                d as *mut u64, 
                e as *mut u64, 
                converted2.as_mut_ptr(), 
                otherempty.as_mut_ptr()) == 0
        }
    }
}
