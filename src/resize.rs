use crate::{RRDCommand, convert_to_mutmutchar, rrd_resize};
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "resize";

#[derive(Debug, Default, Clone)]
pub struct Command {
    // for whatever reason, resize is very strict about exactly what it wants
    // nothing is optional.
    pub argc: i32,
    pub argv: Vec<String>,
    pub infilename: String,      // RRD to operate on
    pub target_rra: String,      // Number of RRA within the RRD
    pub grow_or_shrink: String,  // "GROW" or "SHRINK"
    pub rows: String             // number of rows to add/remove 
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(infilename: String) -> Builder {
        let data = Command { infilename, ..Default::default()};
        Builder { data }
    }
    
    // We gotta be very specific about ordering argv
    // 0: ARGV0, 1: infilename, 2: target_rra, 3: g_or_s, 4: rows
    pub fn build(self) -> Command {
        let mut retval = self.data.clone();
        retval.argv.push(ARGV0.to_string());
        retval.argv.push(retval.infilename.clone());
        retval.argv.push(retval.target_rra.clone());
        retval.argv.push(retval.grow_or_shrink.clone());
        retval.argv.push(retval.rows.clone());
        retval.argc = retval.argv.len() as i32;
        retval
    }

    // !! EVERYTHING IS REQUIRED !!
    pub fn with_rra_num(mut self, var: i32) -> Builder {
        self.data.target_rra = var.to_string();
        self
    }

    pub fn with_function(mut self, var: G_OR_S) -> Builder {
        self.data.grow_or_shrink = var.to_string();
        self
    }

    pub fn with_num_rows(mut self, var: i32) -> Builder {
        self.data.rows = var.to_string();
        self
    }
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {
        unsafe {
            let mut converted = convert_to_mutmutchar(self.argv.clone());
            rrd_resize(self.argc as c_int, converted.as_mut_ptr()) == 0
        }
    }
}

pub enum G_OR_S { GROW, SHRINK }
impl std::fmt::Display for G_OR_S {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            G_OR_S::GROW   => write!(f, "GROW"),
            G_OR_S::SHRINK => write!(f, "SHRINK")
        }
    }
}