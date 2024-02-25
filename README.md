# librrd-sys
Rust FFI library wrapping RRDTool's librrd

## Current Functionality
* Create, Dump, Graph (see `examples/`` directory)
    * Graph generates the image but then segfauts; still don't know why, putting it on the back-burner to finish implementing the other 11 subcommands

## To Do
* Export, Fetch, First, Info, Last, Lasdupdate, List, Resize, Restore, Tune, Update

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


librrd-sys v0.1.44 2024-Feb-25