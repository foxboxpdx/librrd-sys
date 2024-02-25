use librrd_sys::first::Builder;
use librrd_sys::get_rrd_error;
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if result == 0 {
        println!("Error retrieving first timestamp RRD: {}", get_rrd_error());
    }
}