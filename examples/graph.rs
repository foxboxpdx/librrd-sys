use librrd_rs::graph::Builder;
use librrd_rs::{get_rrd_error, RRAType, RRDCommand};
fn main() {
    let command = Builder::new("createtest.png".to_string())
        .start("-1d")
        .title("Test Graph")
        .width(800)
        .height(200)
        .with_def("t", "createtest.rrd", "temperature", RRAType::AVERAGE)
        .with_line("t", "#00FF00", "temperature")
        .with_gprint("t", RRAType::AVERAGE, "Temp avg %6.1lf C")
        .build();
    let result = command.execute();
    println!("Result: {}", result);
    if !result {
        println!("Error creating Graph: {}", get_rrd_error());
    }
}