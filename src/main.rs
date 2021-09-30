mod utils;
mod create;
mod publish;
mod check;
use ansi_term::Colour::Green;
use create::entry::createNew;
use utils::shared::readCli;
use utils::terminal;
use publish::entry::publishLocale;
use check::entry::checkTranslation;



fn main() {
    terminal::green("Hello there!");
    terminal::cyan("Thank you for helping us translate Fileglass, let's begin!");
    terminal::cyan("First, please choose what would you like to do.");
    println!(">> {} - Create new translation in a selected language", Green.paint("new"));
    println!(">> {} - Publish a translation file for review to the Fileglass team", Green.paint("publish"));
    println!(">> {} - Check for missing keys in you locale", Green.paint("check"));
    let mut line = String::new();
    readCli(&mut line);
    line = line.trim().to_string();
    if line == "new" {
        createNew();
    } else if line == "publish" { //prime example of how not to make CLIs
        publishLocale();
    } else if line == "check" {
        checkTranslation();
    }
}
