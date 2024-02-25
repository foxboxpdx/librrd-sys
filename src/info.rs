use crate::{rrd_info, rrd_info_print, rrd_info_free, rrd_info_t};
use crate::{RRDCommand, convert_to_mutmutchar};
use std::collections::HashMap;
use std::ffi::c_int;

// What should always get added to argv first
static ARGV0: &str = "info";

// Define a struct that can be passed to an 'rrdtool info' call
#[derive(Debug, Default, Clone)]
pub struct Command {
    pub argc: i32,
    pub argv: Vec<String>,
    pub filename: String,
    pub flags: Vec<String>, // For optionss with no args
    pub opts: HashMap<String, String> // for options with required args
}

impl RRDCommand for Command {
    fn execute(&self) -> bool {true}
}

pub struct Builder {
    pub data: Command
}

impl Builder {
    pub fn new(filename: String) -> Builder {
        let data = Command { filename, ..Default::default()};
        Builder { data }
    }

    pub fn daemon(&mut self) {
        self.data.flags.push("--daemon".to_string());
    }

    pub fn noflush(&mut self) {
        self.data.flags.push("--noflush".to_string());
    }

    // Take any flags/opts set, combine them into a single
    // long string, push stuff into argv, set the appropriate
    // value to argc, and return the finished Command
    pub fn build(&self) -> Command {
        let mut retval = self.data.clone();
        let mut options = retval.flags.join(" ");
        for (k, v) in &self.data.opts {
            let s = format!(" {} {}", k, v);
            options.push_str(&s);
        }
        retval.argv.push(ARGV0.to_string());
        retval.argv.push(options);
        retval.argc = retval.argv.len() as i32;
        retval
    }
}

// Options:
// daemon, noflush
// I don't like passing around this *mut rrd_info_t but the struct
// contains a pointer to another rrd_info_t and I just don't even
pub fn info(argc: i32, argv: Vec<String>) -> *mut rrd_info_t {
    unsafe {
        let mut conv = convert_to_mutmutchar(argv);
        rrd_info(argc as c_int, conv.as_mut_ptr())
    }
}

pub fn info_print(data: *mut rrd_info_t) {
    unsafe {
        rrd_info_print(data);
    }
}

pub fn info_free(data: *mut rrd_info_t) {
    unsafe {
        rrd_info_free(data);
    }
}

/*
// I don't think this is needed
pub fn info_push(mut info: rrd_info_t, key: String, type_: rrd_info_type_t, value: rrd_infoval_t) -> *mut rrd_info_t {
    unsafe {
        let c_str = CString::new(key).unwrap();
        let cv: Vec<u8> = c_str.into_bytes_with_nul();
        let mut tmp: Vec<i8> = cv.into_iter().map(|c| c as i8).collect::<_>(); // line 7
        let c_key: *mut i8 = tmp.as_mut_ptr();
        //let c_key: *const c_char = c_str.as_ptr() as *const c_char;
        rrd_info_push(&mut info, c_key, type_, value)
    }
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trygetinfo() {
        let argv: Vec<String> = vec!["info".to_string(), 
                                     "test.rrd".to_string()];
        let argc = 2;
        let info = info(argc, argv);
        info_print(info);
        assert!(true);
    }
}