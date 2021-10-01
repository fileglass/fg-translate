use reqwest;
use std::fs::metadata;

pub fn readCli(mut line: &mut String) {
    std::io::stdin().read_line(&mut line).unwrap();
}


pub fn fetchLocale(key: &str) -> Result<String, reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    );
    let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build().unwrap();
    let text = client.get(format!("https://api.file.glass/v3/common/translation?access_key={}", key)).send().unwrap().text().unwrap();
    Ok(text)
}

pub fn isPathAndTsFile(p: &str) -> bool {
    let md = metadata(p).expect("Invalid path!");

    md.is_file() && p.ends_with(".ts")
}