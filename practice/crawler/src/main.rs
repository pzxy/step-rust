use log::{info};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // 爬谁?
    let res = reqwest::get("https://solution.tuya.com/api/v2/market/searchList?pageNo=1&pageSize=100").await?.text().await?;
    let resp: Root = serde_json::from_str(res.as_str())?;
    if resp.success == true {
        for v in resp.result.datas {
            info!("{} --> {}", v.solution_name, v.head_line)
        }
    }
    // 存储
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct2 {
    pub code: String,
    pub level: i64,
    pub name: String,
    pub category_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Struct1 {
    pub on_shelf: i64,
    pub solution_name: String,
    pub sort: i64,
    pub gmt_create: String,
    pub solution_code: String,
    pub head_line: String,
    pub main_image: String,
    pub category_list: Vec<Struct2>,
    pub solution_id: i64,
    pub solution_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Result {
    pub datas: Vec<Struct1>,
    pub page_no: i64,
    pub search_title: String,
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    pub result: Result,
    pub t: i64,
    pub success: bool,
    pub status: String,
}