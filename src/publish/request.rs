use reqwest;

pub fn uploadFile(apiKey: &str, filePath: &str) -> Result<String, reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    );
    let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build().unwrap();
    let form = reqwest::blocking::multipart::Form::new().file("file", filePath).unwrap();
    let res = client.post(format!("https://api.file.glass/v3/common/translation/upload?access_key={}", apiKey))
        .multipart(form)
        .send()?;
    res.text()


}