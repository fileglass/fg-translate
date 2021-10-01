use crate::utils::shared::readCli;
use crate::utils::terminal;
use crate::utils::reader::readPath;
use crate::utils::validator::isCodeValid;
use crate::publish::fromname::getLocaleFromName;
use crate::publish::request::uploadFile;
use crate::utils::shared::isPathAndTsFile;
fn upload(path: &str) {
    let mut apiKey = String::new();
    terminal::cyan("Please input your Fileglass API key to upload the translation file.");
    readCli(&mut apiKey);
    apiKey = apiKey.trim().to_string();
    let resp = uploadFile(&apiKey, &path).unwrap();
    if resp.contains("failed") {
        terminal::red(&format!("Request failed with reason: {}", &resp));
        upload(&path);
    } else {
        if resp == "Submitted" {
            terminal::green("Your translation has been submitted, we will review it soon and notify you.");
            terminal::green("Thank you for your contribution! <3");
        } else {
            terminal::green(&resp);
        }
    }
}

pub fn publishLocale() {
    let mut path = String::new();
    terminal::cyan("Please input the path to your translation file.");
    readCli(&mut path);
    path = path.trim().to_string();
    if !isPathAndTsFile(&path) {
        terminal::red(&format!("{} is not a TypeScript file!", path));
        return publishLocale();
    }
    let file = readPath(&path);
    let name = getLocaleFromName(&path);
    if name.len() == 0 {
        terminal::yellow("The locale name is not parsable from your filename, please rename it like locale_code.ts");
        return publishLocale()
    }
    if !isCodeValid(&name) {
        terminal::red(&format!("{} is not a valid language code! \n \n \n \n", name));
        return publishLocale()
    }

    if file.contains("''") {
        terminal::yellow("WARNING! Your file contains empty entries.")
    }
    terminal::cyan(&format!("Locale code: {} (if you don't see the correct shorthand code here, then your file is not named correctly).", name));
    upload(&path);

}