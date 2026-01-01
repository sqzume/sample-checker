use scraper::{Html, Selector};
use std::fs;
use std::fs::write;

fn get_contest_sample(url: &str, dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // url に対して HTTP リクエスト
    let response = reqwest::blocking::get(url)?;

    // 200 OK のとき以外はスキップ
    if !response.status().is_success() {
        return Ok(());
    }

    // HTML を取得
    let html_content = response.text()?;

    // html を静的解析する
    let document = Html::parse_document(&html_content);

    // どのタグ・クラスの内容を抽出するか決定する
    let section_selector = Selector::parse("span.lang-ja div.part section").unwrap();
    let h3_selector = Selector::parse("h3").unwrap();
    let pre_selector = Selector::parse("pre").unwrap();

    // section_selector をもとに取得した内容を順に見ていく
    for section in document.select(&section_selector) {
        // h3_selector に合う場合
        if let Some(h3_element) = section.select(&h3_selector).next() {
            let h3_text = h3_element.text().collect::<Vec<_>>().join("");

            // h3 が "入力例" または "出力例" で始まるかどうか
            if h3_text.starts_with("入力例") || h3_text.starts_with("出力例") {
                if let Some(pre_element) = section.select(&pre_selector).next() {
                    // サンプルを取得
                    let pre_text = pre_element.text().collect::<Vec<_>>().join("");

                    // 何番目かどうかを取得
                    let num = h3_text.split_whitespace().last().unwrap_or("0");

                    // サンプルをファイルとして保存
                    if h3_text.starts_with("入力例") {
                        let file_path = format!("{}/sample_{}.in", dir_path, num);
                        write(file_path, &pre_text)?;
                    }
                    if h3_text.starts_with("出力例") {
                        let file_path = format!("{}/sample_{}.out", dir_path, num);
                        write(file_path, &pre_text)?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn run(arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ディレクトリを作成
    fs::create_dir_all("tests")?;

    // A~G 問題までのサンプルを取得
    for i in 'a'..'g' {
        let url = format!("https://atcoder.jp/contests/{}/tasks/{}_{}", arg, arg, i);

        let file_path = format!("tests/{}", i);
        fs::create_dir_all(&file_path)?;

        get_contest_sample(&url, &file_path)?;
    }

    Ok(())
}
