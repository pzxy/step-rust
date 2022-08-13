use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 爬谁?
    let client = reqwest::Client::new();
    let res = client.get("https://solution.tuya.com/api/v2/market/searchList?pageNo=12&pageSize=10")
        .send().await?;
    // 正则
    let resp = res.text().await?;

    let mut f = File::create("data.json")?;
    let _ = f.write(resp.as_ref());
    Ok(())
}

struct Struct1 {
    pub code: String,
    pub level: i64,
    pub name: String,
    pub category_id: i64,
}

struct Struct {
    pub on_shelf: i64,
    pub solution_name: String,
    pub sort: i64,
    pub gmt_create: String,
    pub solution_code: String,
    pub head_line: String,
    pub main_image: String,
    pub category_list: Vec<Struct1>,
    pub solution_id: i64,
    pub solution_type: i64,
}

struct Result {
    pub datas: Vec<Struct>,
    pub page_no: i64,
    pub search_title: String,
    pub total_count: i64,
}

struct Root {
    pub result: Result,
    pub t: i64,
    pub success: bool,
    pub status: String,
}