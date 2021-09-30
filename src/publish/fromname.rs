use regex::Regex;

pub fn getLocaleFromName(name: &str) -> String {
    let re = Regex::new(r"locale_(.+)\.ts").unwrap();
    let caps = re.captures(&name).unwrap();
    caps.get(1).map_or("", |m| m.as_str()).to_string()
}