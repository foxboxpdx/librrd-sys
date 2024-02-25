use librrd_sys::restore::Builder;
use librrd_sys::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new("createtest.xml".to_string(), "createtest.rrd".to_string())
        .range_check()
        .force_overwrite()
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error restoring RRD: {}", get_rrd_error());
    }
}