use librrd_sys::fetch::Builder;
use librrd_sys::{get_rrd_error, RRDCommand, RRAType};
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .resolution("15m")
        .start("-1h")
        .with_cf(RRAType::AVERAGE)
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error fetching RRD: {}", get_rrd_error());
    }
}