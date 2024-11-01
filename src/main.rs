use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Amount of expense - value greater than 0.0
    #[arg(short, long, default_value_t = 0.0)]
    amount: f32,

    /// Amounts of expense - value greater than 0.0
    #[arg(short = 'z', long, default_value_t = 0.0)]
    amounts: f32,
}

fn main() {
    let args = Args::parse();

    println!("Hello {:?}!", args.amount);
}
