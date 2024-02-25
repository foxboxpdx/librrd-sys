use librrd_rs::fetch::Builder;
use librrd_rs::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .resolution("15m")
        .start("-1h")
        .with_cf(librrd_rs::RRAType::AVERAGE)
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error fetching RRD: {}", get_rrd_error());
    }
}