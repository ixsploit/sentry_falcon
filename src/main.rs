pub mod config;
pub mod controller;
pub mod organization;
pub mod s3;
pub mod sts;
pub mod iam;
pub mod route53;
pub mod aws_config;
pub mod rds;
pub mod kms;

use clap::{arg,Command};
use std::process;

fn cli() -> Command<'static> {
    Command::new("sentryfalcon")
        .about("manage you aws accounts")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("s3").about("manages s3 resources"))
        .subcommand(Command::new("rds").about("manages rds resources"))
        .subcommand(Command::new("kms").about("manages kms resources"))
        .subcommand(Command::new("zone").about("list route53 zones"))
        .subcommand(Command::new("subdomains").about("list route53 subdomains"))
        .subcommand(Command::new("resources").about("lists all resources"))
        .subcommand(Command::new("id")
            .about("account id lookup")
            .arg(arg!([ID]))
        )
        .subcommand(Command::new("init").about("initalizes new config"))
}

#[tokio::main]
pub async fn main() {
    let matches = cli().get_matches();
  
    let config_result = config::Config::new();
    if config_result.is_err() {
        eprintln!("Config Error: \n");
        eprintln!("{}", config_result.err().unwrap());
        process::exit(1);
    } 
    let config = config_result.unwrap();
    //add organization accounts to config
    let config = organization::get_accounts(config).await;
    if config.is_err() {
        eprintln!("Config Error: \n");
        eprintln!("{}", config.as_ref().err().unwrap());
    }
    let config = config.unwrap();
    match matches.subcommand() {
        Some(("s3", sub_matches)) => {
            println!("S3");
            println!("{:?}", config);
            let result = controller::run_account(config, s3::list).await;
            if result.is_err() {
                eprintln!("s3 controller error:\n");
                eprintln!("{}", result.err().unwrap());
            }
        }
        Some(("rds", sub_matches)) => {
            println!("rds");
            println!("{:?}", config);
            let result = controller::run_region(config, rds::list).await;
            if result.is_err() {
                eprintln!("s3 controller error:\n");
                eprintln!("{}", result.err().unwrap());
            }
        }
        Some(("kms", sub_matches)) => {
            println!("kms");
            println!("{:?}", config);
            let result = controller::run_region(config, kms::list).await;
            if result.is_err() {
                eprintln!("s3 controller error:\n");
                eprintln!("{}", result.err().unwrap());
            }
        }
        Some(("subdomains", sub_matches)) => {
            let result = controller::run_account(config, route53::get_subdomains).await;
            if result.is_err() {
                eprintln!("s3 controller error:\n");
                eprintln!("{}", result.err().unwrap());
            }
        }
        Some(("zone", sub_matches)) => {
            let result = controller::run_account(config, route53::get_domains).await;
            if result.is_err() {
                eprintln!("s3 controller error:\n");
                eprintln!("{}", result.err().unwrap());
            }
        }
        Some(("resources", sub_matches)) => {
            let result = controller::run_region(config, aws_config::show_resources).await;
            if result.is_err() {
                eprintln!("s3 controller error:\n");
                eprintln!("{}", result.err().unwrap());
            }
        }
        Some(("id", sub_matches)) => {
            println!("id");
            println!("{:?}", config);
            let result = organization::get_account_name(&sub_matches.value_of("ID").unwrap_or("0").to_string(), &config).await;
            if result.is_err() {
                eprintln!("Organziation controller error:\n");
                eprintln!("{}", result.err().unwrap());
            } else {
                println!("{}", result.ok().unwrap());
            }
        }
        Some(("init", sub_matches)) => {
            let result = config::init();
            if result.is_err() {
                println!("Init Error: \n");
                println!("{}", result.err().unwrap());
                process::exit(1);
            }
        }
        _ => println!("Invalid command"),
    }
}
