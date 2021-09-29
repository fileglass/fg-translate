use reqwest;
mod validator;
mod writer;
mod formatter;
fn readCli(mut line: &mut String) {
   std::io::stdin().read_line(&mut line).unwrap();
}

fn fetchLocale(key: &str) -> Result<String, reqwest::Error> {
    let text = reqwest::blocking::get(format!("https://api.file.glass/v3/common/translation?access_key={}", key))?.text()?;
    Ok(text)
}



fn getLocale(loc: &str) {
    println!("Please input your Fileglass API key to download the translation file");
    let mut apikey = String::new();
    readCli(&mut apikey);
    let mut locale = fetchLocale(&apikey).unwrap();
    if locale.contains("ERR_INVALID_APIKEY") { //bad solution
        println!("Invalid API key! \n");
        getLocale(&loc);
    } else {
        println!("Locale downloaded, please input a path where you want your files to be saved.");
        let mut path = String::new();
        readCli(&mut path);
        path = path.trim().to_string();
        writer::writeToPath(&format!("{}/locale_en.ts", path), &locale);
        locale = str::replace(&locale, "TRANSLATIONS_EN", &format!("TRANSLATIONS_{}", loc.to_uppercase()));
        let replaced = formatter::createEmptyEntries(&mut locale);
        writer::writeToPath(&format!("{}/locale_{}.ts", path, loc), &replaced);
    }
}


fn main() {
    println!("Hello there!");
    println!("Thank you for helping us translate Fileglass, let's begin!");
    let mut line = String::new();
    println!("What language would you like to translate to? (use the shorthands, refer to: https://www.w3.org/International/O-charset-lang.html)");
    readCli(&mut line);
    line = line.trim().to_string();
    if line == "en" {
        println!("We already have locales for this language :( \n \n \n \n");
        main();
    } else {
        if validator::isCodeValid(&line) {
            getLocale(&line);
        } else {
            println!("{} is not a valid langauge code! \n \n \n \n", line);
            main()
        }
    }
}
