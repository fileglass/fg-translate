use ansi_term::Colour;

pub fn red(text: &str) {
println!("{}", Colour::Red.paint(text));
}

pub fn green(text: &str) {
    println!("{}", Colour::Green.paint(text));
}

pub fn yellow(text: &str) {
    println!("{}", Colour::Yellow.paint(text));
}

pub fn cyan(text: &str)  {
    println!("{}", Colour::Cyan.paint(text));
}