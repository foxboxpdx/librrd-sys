use librrd_sys::list::Builder;
fn main() {
    let command = Builder::new("/path/to/rrcached/base".to_string())
        .recursive()
        .build();
    let result = command.execute();
    println!("Result: {}", result);
}