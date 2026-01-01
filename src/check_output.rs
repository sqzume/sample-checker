use std::{
    fs::{self, File, read_to_string},
    process::{Command, Stdio},
};

fn count_files(problem: &str) -> Result<usize, std::io::Error> {
    let path = format!("tests/{}", problem);
    let count = fs::read_dir(path)?.count();
    Ok(count)
}

pub fn run(problem: &str) -> Result<(), Box<dyn std::error::Error>> {
    let files_num = count_files(problem)?;
    let path = format!("tests/{}", problem);

    let in_path = format!("{}/sample_1.in", path);
    let out_path = format!("{}/sample_1.out", path);

    // in.txt を読み込む
    let input_content = read_to_string(&in_path)?;
    // out.txt を読み込む
    let output_content = read_to_string(&out_path)?;

    // in.txt を標準入力として a.out を実行
    // [TODO] パスの指定方法をカレントディレクトリに基づくようにする
    let input = File::open(&in_path)?;
    let output = Command::new("test_sample/a.out")
        .stdin(Stdio::from(input))
        .output()?;

    // サンプルケースを表示
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

    // out.txt と実行結果を比較して、"AC" or "WA" を出力する
    if output_content.trim() == String::from_utf8_lossy(&output.stdout).trim() {
        println!("{}{}{}", "\x1b[32m", "AC", "\x1b[0m");
    } else {
        println!("{}{}{}", "\x1b[31m", "WA", "\x1b[0m");
    }

    Ok(())
}
