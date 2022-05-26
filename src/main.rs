pub mod functions;
use clap::Parser;
use functions::structures::TransactionType;
use functions::transaction_update::transaction_update;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'u', long = "user")]
    user: String,
    #[clap(short = 't', long = "transaction", default_value = "credit")]
    transaction_type: String,
    #[clap(short = 'a', long = "amount")]
    amount: String,
}

fn main() {
    let args = Cli::parse();
    let amount_in_float: f32 = args.amount.parse().expect("Invalid Amount");
    let transaction_type = args.transaction_type.parse::<TransactionType>().unwrap();
    transaction_update(&args.user, transaction_type, amount_in_float);
    println!("Transaction successful");
}
