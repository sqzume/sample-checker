mod check_output;
mod get_sample;

use std::{env, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 引数を受け取る
    let args: Vec<String> = env::args().collect();

    // 引数がない場合、終了する
    if args.len() == 1 || args.len() == 2 {
        eprintln!("[Error]: 引数を指定してください");
        process::exit(1);
    }

    if args[1] == "g" {
        get_sample::run(&args[2])?;
    }

    if args[1] == "t" {
        check_output::run(&args[2])?;
    }

    Ok(())
}
