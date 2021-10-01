use crate::utils::shared::readCli;
use crate::utils::shared::fetchLocale;
use crate::utils::writer;
use crate::utils::terminal;
use crate::create::formatter;
use crate::utils::validator;

fn getLocale(loc: &str) {
    terminal::cyan("Please input your Fileglass API key to download the translation file.");
    let mut apikey = String::new();
    readCli(&mut apikey);
    let mut locale = fetchLocale(&apikey).unwrap();
    if locale.contains("ERR_INVALID_APIKEY") { //bad solution
        terminal::red("Invalid API key! \n");
        getLocale(&loc);
    } else {
        terminal::green("Locale downloaded, please input a path where you want your files to be saved.");
        let mut path = String::new();
        readCli(&mut path);
        path = path.trim().to_string();
        writer::writeToPath(&format!("{}/locale_en.ts", path), &locale);
        locale = str::replace(&locale, "TRANSLATIONS_EN", &format!("TRANSLATIONS_{}", loc.to_uppercase()));
        let replaced = formatter::createEmptyEntries(&mut locale);
        writer::writeToPath(&format!("{}/locale_{}.ts", path, loc), &replaced);
        terminal::green("Finished, happy translating! <3");
    }
}

pub fn createNew() {
    let mut line = String::new();
    terminal::cyan("What language would you like to translate to? (use the shorthands, refer to: https://gist.github.com/ndbroadbent/588fefab8e0f1b459fcec8181b41b39c)");
    readCli(&mut line);
    line = line.trim().to_string();
    if line == "en" {
        terminal::yellow("We already have locales for this language :( \n \n \n \n");
        createNew();
    } else {
        if validator::isCodeValid(&line) {
            getLocale(&line);
        } else {
            terminal::red(&format!("{} is not a valid langauge code! \n \n \n \n", line));
            createNew()
        }
    }
}