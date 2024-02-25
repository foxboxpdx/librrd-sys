# librrd-sys
Rust FFI library wrapping RRDTool's librrd

## Current Functionality
* ✅ `create`
* ✅ `dump`
* ⛔️ `export` - Segfault
* ⛔️ `fetch` - Segfault
* ⛔️ `graph` - Generates image but then segfaults
* ✅ `info`
* ✅ `last`
* ✅ `lastupdate`
* 🔶 `list` - Needs further testing, should work
* ✅ `resize`
* 🔶 `restore` - Needs further testing, should work
* ✅ `tune`
* ✅ `update`

## Usage
Check the provided examples for very basic usage.  Check the associated RRDTool man page for complete usage info.

## To Do
* Try to fix `export`, `fetch`, and `graph`
* Implement `updatev`
* Set up rrdcached to fully test `list`
* Set up an xml file to fully test `restore`
* Maybe implement the stubs in `misc.rs`

## Building
* Edit `wrapper.h` to point at the `rrd.h` and `rrd_client.h` header files on your system.
    * Need to see if I can look up a way to automate this based on detected system triple
* Edit `build.rs` to point at the location of the precompiled `rrd` C libraries
    * Need to try to automate this too
* Build away!

## Examples
* Start with `create` which will generate a very simple RRD file with one data store and one RRA.  This should appear in the base directory as `createtest.rrd`.
* Everything else should just use that RRD file to do perform its namesake functionality.

## Help
* FFI is Rust hard-mode and I have no idea how to properly test this stuff or turn it into a Crate.  If by some chance a fellow rustacean finds this repo, I am certainly open to advice/assistance.


librrd-sys v0.1.79 2024-Feb-25