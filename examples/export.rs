use librrd_rs::export::Builder;
use librrd_rs::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new()
        .start("now-1h")
        .end("now")
        .with_def("t", "test.rrd", "temperature", librrd_rs::RRAType::AVERAGE)
        .with_xport("t", "some stuff")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error exporting RRD: {}", get_rrd_error());
    }
}