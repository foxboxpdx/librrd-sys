use librrd_rs::lastupdate::Builder;
use librrd_rs::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error retrieving lastupdate from RRD: {}", get_rrd_error());
    }
}