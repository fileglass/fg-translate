use reqwest;
use std::fs::metadata;

pub fn readCli(mut line: &mut String) {
    std::io::stdin().read_line(&mut line).unwrap();
}


pub fn fetchLocale(key: &str) -> Result<String, reqwest::Error> {
    let text = reqwest::blocking::get(format!("https://api.file.glass/v3/common/translation?access_key={}", key))?.text()?;
    Ok(text)
}

pub fn isPathAndTsFile(p: &str) -> bool {
    let md = metadata(p).expect("Invalid path!");

    md.is_file() && p.ends_with(".ts")
}