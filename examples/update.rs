use librrd_sys::update::Builder;
use librrd_sys::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .with_update("-10:5:10")
        .with_update("N:15:20")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error updating RRD: {}", get_rrd_error());
    }
}