use reqwest;

pub fn readCli(mut line: &mut String) {
    std::io::stdin().read_line(&mut line).unwrap();
}


pub fn fetchLocale(key: &str) -> Result<String, reqwest::Error> {
    let text = reqwest::blocking::get(format!("https://api.file.glass/v3/common/translation?access_key={}", key))?.text()?;
    Ok(text)
}