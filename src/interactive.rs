use crate::{convert, print_request};
use std::io::Write;

/// Starts a User friendly interactive mode.
///
/// It guides the user through the currency conversion process step by step.
pub fn assist_mode() {
    main_menu();
}

/// Build a menu and ensures that the user select a command.
fn show_command(commands: &[&str]) -> String {
    let mut option = String::new();

    loop {
        println!();
        println!("*** Commands ***");
        for (id, command) in commands.iter().enumerate() {
            println!("{}. {command}", id + 1);
        }
        println!("0. Exit");

        print!("option> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut option).unwrap();

        if let Ok(option) = option.trim().parse() {
            if (0..=commands.len()).contains(&option) {
                break;
            }
        }

        eprintln!("Invalid option!");
    }

    option
}

/// Main menu.
fn main_menu() {
    loop {
        let option: String = show_command(&[
            "Currency exchange",
            "List of currencies (Exchange rate base on USD)",
        ]);
        match option.trim() {
            "1" => currency_exchange(),
            "2" => print_request("currency-list")
                .map_err(|err| eprintln!("{err}"))
                .unwrap(),
            "0" => return,
            _ => eprintln!("Invalid option!"),
        }
    }
}

/// Currency exchange interaction.
fn currency_exchange() {
    let mut source = String::new();
    let mut target = String::new();
    let mut amount = String::new();

    loop {
        println!();
        println!("*** Currency Exchange ***");
        print!("Source currency (eg. USD): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut source).unwrap();
        print!("Target currency (eg. EUR): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut target).unwrap();
        print!("Amount (eg. 100.0): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut amount).unwrap();

        if let Ok(amount) = amount.trim().parse::<f32>() {
            match convert(source.trim(), target.trim(), amount) {
                Ok((amount, rate)) => println!(
                    "With a current exchange rate of {}, the target amount is {}.",
                    rate, amount
                ),
                Err(err) => eprintln!("{err}"),
            }
            break;
        }

        eprintln!("An decimal amount is expected!");
    }
}
