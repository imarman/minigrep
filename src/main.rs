use std::process;
use clap::Parser;
use minigrep::{Config, run};
use crate::cli::Cli;

mod cli;


fn main() {

    let cli_args = Cli::parse();

    let conf = Config::build_with_cli(cli_args.keyword, cli_args.file_path, cli_args.ignore_case).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    // 如果有异常了 就返回错误
    if let Err(e) = run(conf) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}