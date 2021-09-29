pub fn isCodeValid(code: &str) -> bool {
    let validCodes: [&str; 45] = ["af","sq","ar","eu","bg","be","ca","hr","cs","da","nl","en","eo","et","fo","fi","fr","gl","de","el","iw","hu","is","ga","it","ja","ko","lv","lt","mk","mt","no","pl","pt","ro","ru","gd","sr","sr","sk","sl","es","sv","tr","uk"];
    validCodes.contains(&code.trim())
}