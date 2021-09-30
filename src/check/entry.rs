use crate::utils::terminal;
use crate::utils::shared::readCli;
use crate::utils::shared::fetchLocale;
use crate::utils::reader::readPath;
use crate::utils::validator::isCodeValid;
use crate::publish::fromname::getLocaleFromName;


fn getKey(file: &str) {
    terminal::cyan("Please input your Fileglass API key to download the translation file.");
    let mut apikey = String::new();
    readCli(&mut apikey);
    let mut locale = fetchLocale(&apikey).unwrap();
    if locale.contains("ERR_INVALID_APIKEY") { //bad solution
        terminal::red("Invalid API key! \n");
        getKey(&file);
    }
    terminal::green("Locale fetched, checking...");

}

pub fn checkTranslation() {
    let mut path = String::new();
    terminal::cyan("Please input the path to your translation file.");
    readCli(&mut path);
    path = path.trim().to_string();
    let file = readPath(&path);
    let name = getLocaleFromName(&path);
    if !isCodeValid(&name) {
        terminal::red(&format!("{} is not a valid language code! \n \n \n \n", name));
        return checkTranslation()
    }

    if file.contains("''") {
        terminal::yellow("WARNING! Your file contains empty entries.")
    }
    getKey(&file);

}