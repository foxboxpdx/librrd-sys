use crate::{convert_to_mutmutchar, rrd_create, DSType, RRAType, RRDCommand};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "create";

// Define a struct that can be passed to an 'rrdtool create' call
#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String>, // for options with required args
    pub sources: Vec<String>, // Data sources
    pub rras: Vec<String> // RoundRobinArchives
}

pub struct Builder {
    pub data: Command
}

// Somewhere in here we gotta have a function for the data definitions
// Maybe an enum for the types? idk.
impl Builder {
    pub fn new(filename: &str) -> Builder {
        let data = Command { filename: filename.to_string(), ..Default::default()};
        Builder { data }
    }

    // req
    pub fn start(mut self, var: &str) -> Builder {
        self.data.opts.insert("--start".to_string(), var.to_string());
        self
    }

    // req
    pub fn step(mut self, var: &str) -> Builder {
        self.data.opts.insert("--step".to_string(), var.to_string());
        self
    }

    // req
    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // req
    pub fn source(mut self, var: &str) -> Builder {
        self.data.opts.insert("--source".to_string(), var.to_string());
        self
    }

    // req
    pub fn template(mut self, var: &str) -> Builder {
        self.data.opts.insert("--template".to_string(), var.to_string());
        self
    }

    // flag
    pub fn no_overwrite(mut self) -> Builder {
        self.data.flags.push("--no-overwrite".to_string());
        self
    }

    // Data sources
    pub fn with_ds(mut self, name: &str, dst: DSType, heartbeat: &str, min: &str, max: &str) -> Builder {
        let s = format!("DS:{}:{}:{}:{}:{}", name, dst.to_string(), heartbeat, min, max);
        self.data.sources.push(s);
        self
    }

    // RRAs
    pub fn with_rra(mut self, rra: RRAType, xff: &str, steps: &str, rows: &str) -> Builder {
        let r = format!("RRA:{}:{}:{}:{}", rra.to_string(), xff, steps, rows);
        self.data.rras.push(r);
        self
    }

    // rrdtool create filename options datasources rras
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
        retval.argv.append(&mut retval.sources);
        retval.argv.append(&mut retval.rras);
        retval.argc = retval.argv.len() as i32;
        //println!("argc: {}\nargv: {:?}", retval.argc, &retval.argv);
        retval
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_create(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}

