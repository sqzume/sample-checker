use std::{
    fs::{self, File, read_to_string},
    process::{Command, Stdio},
};

fn count_files(problem: &str) -> Result<usize, std::io::Error> {
    let path = format!("tests/{}", problem);
    let count = fs::read_dir(path)?.count();
    Ok(count)
}

fn check_main(sample_path: &str, sample_num: usize) -> Result<(), Box<dyn std::error::Error>> {
    // path を指定
    let in_path = format!("{}/sample_{}.in", sample_path, sample_num);
    let out_path = format!("{}/sample_{}.out", sample_path, sample_num);

    // sample の内容を読み込む
    let input_content = read_to_string(&in_path)?;
    let output_content = read_to_string(&out_path)?;

    // sample を標準入力として a.out を実行
    let input = File::open(&in_path)?;
    let output = Command::new("./a.out").stdin(Stdio::from(input)).output()?;

    // sample を表示
    println!("{}{}{}", "\x1b[34m", "[入力]", "\x1b[0m");
    println!("{}", input_content.trim());
    println!();

    // 実行結果を表示
    println!("{}{}{}", "\x1b[34m", "[実行結果]", "\x1b[0m");
    println!("{}", String::from_utf8_lossy(&output.stdout).trim());
    println!();

    // 正しい答えを表示
    println!("{}{}{}", "\x1b[34m", "[正しい答え]", "\x1b[0m");
    println!("{}", output_content.trim());
    println!();

    // sample と実行結果を比較して、"AC" or "WA" を出力する
    if output_content.trim() == String::from_utf8_lossy(&output.stdout).trim() {
        println!("{}{}{}", "\x1b[32m", "AC", "\x1b[0m");
    } else {
        println!("{}{}{}", "\x1b[31m", "WA", "\x1b[0m");
    }

    Ok(())
}

pub fn run(problem: &str) -> Result<(), Box<dyn std::error::Error>> {
    let files_num = count_files(problem)? / 2;
    let path = format!("tests/{}", problem);

    for i in 1..=files_num {
        check_main(&path, i)?;
    }

    Ok(())
}
