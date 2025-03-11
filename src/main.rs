#[derive(clap::Parser)]
struct Args {
    timestamp: i128,
    resolution: Resolution,
}

enum Resolution {
    Secs,
    Millis,
}

fn main() {
    println!("Hello, world!");
}
