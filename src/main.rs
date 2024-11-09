use std::env;
// use std::process::exit;
// use std::process;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    number: u32,
}

fn main() {
    let current = env::current_exe().unwrap();
    let current_str = current.into_os_string().into_string().unwrap();
    println!("{current_str}");
    let args = Cli::parse();
    let value = args.number;
    let parity: &str = if value % 2 == 0 { "even" } else { "odd" };
    println!("The given value is {parity}");
}

// fn main() {
//     let args = env::args().collect::<Vec<String>>();
//     if args.len() == 1 {
//         println!("No argument given!");
//         exit(1)
//     }
//     if args.len() > 2 {
//         println!("Too many arguments given!");
//         exit(1);
//     }
//     let input = &args[1];
//     let value = str::parse::<u32>(&input);
//     match value {
//         Ok(v) => {
//             let parity: &str = if v % 2 == 0 { "even" } else { "odd" };
//             println!("The given value is {parity}");
//         }
//         Err(_) => {
//             println!("Something has gone terribly wrong...");
//             exit(1);
//         }
//     }
// }
