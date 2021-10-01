use regex::{Regex};

fn handleLine(mut line: &mut String, re: &Regex) -> String {
    *line = line.trim().to_string();
    if !line.contains("{") && !line.contains("/*") && !line.contains("//") && !line.contains("}") && line.len() > 0 && !line.starts_with("'") && !line.starts_with(r#"""#)   {
        let cap = re.captures(line);
        let m = match cap {
            Some(caps) => {
                let mut key = caps.get(1).map_or("", |m| m.as_str()).to_string();
                key = key.replace(":", "");
                let vec = key.split(" ").collect::<Vec<&str>>();
                vec.get(0).unwrap().to_string()
            }
            None => "".to_string()
        };
        m
    } else {
        "".to_string()
    }
}

pub fn getTsObjectKeys(s: &str) -> Vec<String>  {
    let mut lines: Vec<String> = Vec::new();
    let re = Regex::new(r"(\s*.+:)").unwrap();
    for mut line in s.lines() {
        let ln = handleLine(&mut line.to_string(), &re);
        if ln.len() > 0 {
            lines.push(ln);
        }
        ;
    }
    //println!("{}", lines.get(0).unwrap().to_string());
    lines

}