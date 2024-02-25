use librrd_rs::dump::Builder;
use librrd_rs::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd")
        .header("xsd")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error dumping RRD: {}", get_rrd_error());
    }
}