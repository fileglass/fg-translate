use regex::Regex;

pub fn createEmptyEntries(mut tx: &mut String) -> String {
    *tx = tx.replace(r#"""#, "'");
    let re = Regex::new(r#"'.*',"#).unwrap();
    let ret = re.replace_all(tx, r"'',");
    ret.to_string()
}