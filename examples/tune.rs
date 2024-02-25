use librrd_sys::tune::Builder;
use librrd_sys::{get_rrd_error, DSType, RRAType, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .add_ds("humidity", DSType::GAUGE, "2000", "U", "U")
        .add_rra(RRAType::MAX, "0.5", "1", "600")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error tuning RRD: {}", get_rrd_error());
    }
}