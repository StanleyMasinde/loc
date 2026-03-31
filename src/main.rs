use loc::cli;

fn main() {
    if let Err(error) = cli::run() {
        println!("An error occoured: {}", error)
    }
}
