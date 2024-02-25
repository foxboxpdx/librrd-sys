use librrd_sys::export::Builder;
use librrd_sys::{get_rrd_error, RRDCommand, RRAType};
fn main() {
    let command = Builder::new()
        .start("now-1h")
        .end("now")
        .with_def("t", "test.rrd", "temperature", RRAType::AVERAGE)
        .with_xport("t", "some stuff")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error exporting RRD: {}", get_rrd_error());
    }
}