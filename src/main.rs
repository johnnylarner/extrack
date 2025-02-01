use clap::Parser;
use extrack::{run, Extrack};

fn main() {
    let args = Extrack::parse();
    run(args);
}
