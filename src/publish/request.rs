use reqwest;

pub fn uploadFile(apiKey: &str, filePath: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new().file("file", filePath).unwrap();
    let res = client.post(format!("https://api.file.glass/v3/common/translation/upload?access_key={}", apiKey))
        .multipart(form)
        .send()?;
    res.text()


}