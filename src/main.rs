use clap::{Parser, ValueEnum};
use std::process;

/// Nkap CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// Source currency code.
    #[arg(short, long, group = "arg_source", requires = "arg_amount")]
    source: Option<String>,
    /// Target currency code.
    #[arg(short, long, group = "arg_target", requires = "arg_amount")]
    target: Option<String>,
    /// Amount to be converted.
    #[arg(
        short,
        long,
        group = "arg_amount",
        requires = "arg_source",
        requires = "arg_target"
    )]
    amount: Option<f32>,
    /// API information to print on stdout.
    #[arg(long, value_enum)]
    print: Option<PrintRequest>,
    /// Starts the interactive mode.
    #[arg(short, long, action)]
    interactive: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PrintRequest {
    /// Print the list of currencies and their current exchange rate base on USD.
    CurrencyList,
}

fn main() {
    let args = Args::parse();

    if args.interactive {
        nkap::interactive::assist_mode();
    } else if let Some(request) = args.print {
        let request = match request {
            PrintRequest::CurrencyList => "currency-list",
        };

        nkap::print_request(request).unwrap_or_else(|err| {
            eprintln!("Problem during print request: {err}");
            process::exit(1);
        });
    } else if let Some(amount) = args.amount {
        let data = nkap::convert(&args.source.unwrap(), &args.target.unwrap(), amount);
        let (target_amount, exchange_rate) = data.unwrap_or_else(|err| {
            eprintln!("Problem of conversion: {err}");
            process::exit(1);
        });

        println!(
            "With a current exchange rate of {}, the target amount is {}.",
            exchange_rate, target_amount
        );
    }

    process::exit(0);
}
