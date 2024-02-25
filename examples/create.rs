use librrd_sys::create::Builder;
use librrd_sys::{get_rrd_error, DSType, RRAType, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd")
        .start("0")
        .step("1800")
        .with_ds("temperature", DSType::GAUGE, "2000", "U", "U")
        .with_rra(RRAType::AVERAGE, "0.5", "1", "600")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error creating RRD: {}", get_rrd_error());
    }
}