use crate::{RRDCommand, convert_to_mutmutchar, rrd_tune, RRAType, DSType};
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
    pub opts: HashMap<String, String>, // for options with required args
    pub del: Vec<String>, // data sources and/or RRAs to delete
    pub add: Vec<String>, // data sources and/or RRAs to add
    pub rraindex: Vec<String> // add/remove or set number of rows for an RRA
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
        retval.argv.append(&mut retval.del);
        retval.argv.append(&mut retval.add);
        retval.argv.append(&mut retval.rraindex);
        retval.argc = retval.argv.len() as i32;
        retval
    }

    /* All options have required parameters */

    pub fn heartbeat(mut self, var: &str) -> Builder {
        self.data.opts.insert("--heartbeat".to_string(), var.to_string());
        self
    }

    pub fn minimum(mut self, var: &str) -> Builder {
        self.data.opts.insert("--minimum".to_string(), var.to_string());
        self
    }

    pub fn maximum(mut self, var: &str) -> Builder {
        self.data.opts.insert("--maximum".to_string(), var.to_string());
        self
    }

    pub fn data_source_type(mut self, var: &str) -> Builder {
        self.data.opts.insert("--data-source-type".to_string(), var.to_string());
        self
    }

    pub fn data_source_rename(mut self, var: &str) -> Builder {
        self.data.opts.insert("--data-source-rename".to_string(), var.to_string());
        self
    }

    pub fn deltapos(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    pub fn deltaneg(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    pub fn window_length(mut self, var: &str) -> Builder {
        self.data.opts.insert("--window-length".to_string(), var.to_string());
        self
    }

    pub fn failure_threshold(mut self, var: &str) -> Builder {
        self.data.opts.insert("--failure-threshold".to_string(), var.to_string());
        self
    }

    pub fn alpha(mut self, var: &str) -> Builder {
        self.data.opts.insert("--beta".to_string(), var.to_string());
        self
    }

    pub fn beta(mut self, var: &str) -> Builder {
        self.data.opts.insert("--beta".to_string(), var.to_string());
        self
    }

    pub fn gamma(mut self, var: &str) -> Builder {
        self.data.opts.insert("--gamma".to_string(), var.to_string());
        self
    }

    pub fn gamma_deviation(mut self, var: &str) -> Builder {
        self.data.opts.insert("--gamma-deviation".to_string(), var.to_string());
        self
    }

    pub fn smoothing_window(mut self, var: &str) -> Builder {
        self.data.opts.insert("--smoothing-window".to_string(), var.to_string());
        self
    }

    pub fn smoothing_window_deviation(mut self, var: &str) -> Builder {
        self.data.opts.insert("--smoothing-window-deviation".to_string(), var.to_string());
        self
    }

    pub fn aberrant_reset(mut self, var: &str) -> Builder {
        self.data.opts.insert("--aberrant-reset".to_string(), var.to_string());
        self
    }

    pub fn step(mut self, var: &str) -> Builder {
        self.data.opts.insert("--step".to_string(), var.to_string());
        self
    }

    pub fn daemon(mut self, var: &str) -> Builder {
        self.data.opts.insert("--daemon".to_string(), var.to_string());
        self
    }

    // Non-option stuff
    pub fn del_ds(mut self, ds_name: &str) -> Builder {
        let s = format!("DEL:{}", ds_name);
        self.data.del.push(s);
        self
    }

    pub fn add_ds(mut self, name: &str, dst: DSType, heartbeat: &str, min: &str, max: &str) -> Builder {
        let s = format!("DS:{}:{}:{}:{}:{}", name, dst.to_string(), heartbeat, min, max);
        self.data.add.push(s);
        self
    }

    pub fn del_rra(mut self, index: i32) -> Builder {
        let s = format!("DELRRA:{}", index);
        self.data.del.push(s);
        self
    }

    pub fn add_rra(mut self, rra: RRAType, xff: &str, steps: &str, rows: &str) -> Builder {
        let s = format!("RRA:{}:{}:{}:{}", rra.to_string(), xff, steps, rows);
        self.data.add.push(s);
        self
    }

    pub fn rra_index(mut self, index: i32, action: char, count: i32) -> Builder {
        let s = format!("RRA#{}:{}{}", index, action, count);
        self.data.rraindex.push(s);
        self
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_tune(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}