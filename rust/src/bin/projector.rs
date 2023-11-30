use clap::Parser;
use projector_cli::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
