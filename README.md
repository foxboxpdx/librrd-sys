# librrd-sys
Rust FFI library wrapping librrd for interacting with round robin databases created with Tobias Oetiker's rrdtool (https://www.rrdtool.org/).

## Current Functionality (0.1.79)
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
* Check the provided examples for very basic usage.  Check the associated RRDTool man page for complete usage info.

## To Do
* Try to fix `export`, `fetch`, and `graph`
* Implement `updatev`
* Set up rrdcached to fully test `list`
* Set up an xml file to fully test `restore`
* Maybe implement the stubs in `misc.rs`

## Building
* Install `librrd-dev` or the equivalent package for your OS
* Include this crate in your dependencies
```toml
[dependencies]
librrd-sys = "0.1"
```
* `build.rs` should automatically find `librrd` if you're on linux or macos, and should include the appropriate `wrapper.h` variant.

### Windows
* Currently no windows support, will be added soon-ish maybe.

## Examples
* Start with `create` which will generate a very simple RRD file with one data store and one RRA.  This should appear in the base directory as `createtest.rrd`.
* Everything else should just use that RRD file to do perform its namesake functionality.

## Help
* FFI is Rust hard-mode and I'm pretty stuck trying to fix those segfaults.  If by some chance a fellow rustacean finds this repo, I am certainly open to advice/assistance.


librrd-sys v0.1.79 2024-Feb-25