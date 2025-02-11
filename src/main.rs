fn main() {
    match demo_rs::apache_arrow::apache_arrow_demo() {
        Ok(_) => println!("Apache Arrow demo completed successfully."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }

    demo_rs::protobuf::protobuf_demo();
}