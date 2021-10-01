use crate::utils::terminal;
use crate::utils::shared::readCli;
use crate::utils::shared::fetchLocale;
use crate::utils::shared::isPathAndTsFile;
use crate::utils::reader::readPath;
use crate::utils::validator::isCodeValid;
use crate::publish::fromname::getLocaleFromName;
use crate::check::transpile::getTsObjectKeys;

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
    let origKeys = getTsObjectKeys(&locale);
    let currKeys = getTsObjectKeys(&file);
    //println!("{}", origKeys.get(0).unwrap().to_string());
    let mut problems = 0;
    for key in origKeys.iter() {
        if !currKeys.contains(key) {
            terminal::yellow(&format!("Key {} is missing!", key.to_string()));
            problems += 1;
        }
    }
    if (problems == 0) {
        terminal::green(&format!("No missing keys found, your locale is ready to be submitted ({} keys in the original file)", origKeys.len().to_string()));
    } else {
        terminal::yellow(&format!("Found {} missing key(s).", problems.to_string()))
    }

}

pub fn checkTranslation() {
    let mut path = String::new();
    terminal::cyan("Please input the path to your translation file.");
    readCli(&mut path);
    path = path.trim().to_string();
    if !isPathAndTsFile(&path) {
        terminal::red(&format!("{} is not a TypeScript file!", path));
        return checkTranslation();
    }
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