#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::{CString, c_char};
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Each major subcommand gets a module and a builder
// Minor subcomands are collected in 'misc'
pub mod create;
pub mod dump;
pub mod export;
pub mod fetch;
pub mod first;
pub mod graph;
pub mod info;
pub mod last;
pub mod lastupdate;
pub mod list;
pub mod misc;
pub mod resize;
pub mod restore;
pub mod tune;
pub mod update;

pub trait RRDCommand {
    fn execute(&self) -> bool;
}

pub trait RRDBuilder {}

pub fn convert_to_mutchar(input: String) -> *mut i8 {
    let bytes: Vec<u8> = input.into_bytes();
    let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();
    c_chars.push(0); // null terminator
    let ptr: *mut c_char = c_chars.as_mut_ptr();
    ptr
}

pub fn convert_to_mutmutchar(input: Vec<String>) -> Vec<*mut i8> {
    let retval: Vec<*mut c_char> = input.iter().map(|x| CString::new(x.clone()).unwrap().into_raw()).collect();
    retval
}

pub fn convert_to_mutmutmutchar(input: Vec<Vec<String>>) -> Vec<*mut *mut i8> {
    let mut retval: Vec<*mut *mut c_char> = Vec::new();
    for i in input {
        let mut mutmutchar: Vec<*mut c_char> = i.iter().map(|x| CString::new(x.clone()).unwrap().into_raw()).collect();
        retval.push(mutmutchar.as_mut_ptr());
    }
    retval
}

/*
 About Options
 Any function with an argv (except resize) is going to send that
 argv to optparse, which is built to accept and parse options
 as though they were sent on the command line (ie, --optname).
 There are 3 types of options:
 OPTPARSE_NONE - has no arguments - denoted as 'flag'
 OPTPARSE_REQUIRED - requires an argument - denoted as 'req'
 OPTPARSE_OPTIONAL - argument is optional - denotes as 'opt'
 Arguments are all string types (const char *)
 Options are passed in as a single long string
 Order (filenames, opts, other stuff) is specific to each
*/

// Enums and helper functions for datastores and rras

// Enum for data source types
pub enum DSType {
    GAUGE,
    COUNTER,
    DERIVE,
    DCOUNTER,
    DDERIVE,
    ABSOLUTE
}

// Enum for RRA types
pub enum RRAType {
    AVERAGE,
    MIN,
    MAX,
    LAST
}

impl fmt::Display for DSType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DSType::ABSOLUTE => write!(f, "ABSOLUTE"),
            DSType::COUNTER  => write!(f, "COUNTER"),
            DSType::DCOUNTER => write!(f, "DCOUNTER"),
            DSType::DDERIVE  => write!(f, "DDERIVE"),
            DSType::DERIVE   => write!(f, "DERIVE"),
            DSType::GAUGE    => write!(f, "GAUGE")
        }
    }
}

impl fmt::Display for RRAType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RRAType::AVERAGE => write!(f, "AVERAGE"),
            RRAType::LAST    => write!(f, "LAST"),
            RRAType::MAX     => write!(f, "MAX"),
            RRAType::MIN     => write!(f, "MIN"),
        }
    }
}

// An interface to rrd_error
// rrd_error has a rrd_get_context() object
// calling rrd_get_error() should return a char* of the contents of
// CTX->rrd_error
pub fn get_rrd_error() -> String {
    unsafe {
        let err = rrd_get_error();
        let estr = std::ffi::CStr::from_ptr(err);
        let slice = estr.to_str().unwrap();
        format!("{}", slice)
    }
}