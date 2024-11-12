use chrono::{DateTime, Utc};
use clap::{command, Parser};

struct Transaction {
    amount: f32,
    category: String,
    date: DateTime<Utc>,
    description: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Amount of expense - value greater than 0.0
    #[arg(short, long, default_value_t = 0.0)]
    amount: f32,

    /// Type of expense
    #[arg(short = 't', long = "type", default_value_t = String::from("expense"))]
    expense_type: String,

    /// Date of expense - format 2024/11/13
    #[arg(long)]
    date: Option<String>,

    /// Description of expense
    #[arg(long)]
    description: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("Hello {:?}!", args.amount);
    println!("Hello {:?}", args.expense_type);
    println!("Hello {:?}", args.description);
    println!("Hello {:?}", args.date);
}
