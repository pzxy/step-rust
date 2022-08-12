use std::fs::File;
use std::io::Write;
use reqwest::header::USER_AGENT;
// use tokio::fs::File;
// use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 爬谁?
    let client = reqwest::Client::new();
    let res = client.get("https://www.zhipin.com/wapi/zpgeek/search/joblist.json?scene=1&query=&city=101010100&experience=&degree=&industry=&scale=&stage=&position=100101&salary=&multiBusinessDistrict=&page=1&pageSize=30")
        .header(USER_AGENT, "My Rust Program 1.0")
        .send().await?;
    // 正则
    let resp = res.text().await?;
    let mut f = File::create("index.html")?;
    let _ = f.write(resp.as_ref());
    Ok(())
}