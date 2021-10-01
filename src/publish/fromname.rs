use regex::Regex;

pub fn getLocaleFromName(name: &str) -> String {
    let re = Regex::new(r"locale_(.+)\.ts").unwrap();
    let cap = re.captures(&name);
    let caps = match cap {
        Some(c) => {
        return c.get(1).map_or("", |m| m.as_str()).to_string();
        }
        None => {
            return "".to_string()
        }
    };
    caps
}