use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();


    if let Err(e) = ddi::run(args) {
        println!("Error executing program: {}", e);
        std::process::exit(1);
    }
}
