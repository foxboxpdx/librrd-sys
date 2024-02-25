use librrd_sys::resize::{Builder, G_OR_S};
use librrd_sys::{get_rrd_error, RRDCommand};
fn main() {
    let command = Builder::new("createtest.rrd".to_string())
        .with_rra_num(0)
        .with_function(G_OR_S::GROW)
        .with_num_rows(5)
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error resizing RRD: {}", get_rrd_error());
    }
}