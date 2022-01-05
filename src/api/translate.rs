use super::response::Response;
use reqwest;

const YOUDAO_API: &str = "https://fanyi.youdao.com/openapi.do?keyfrom=node-fanyi&key=110811608&type=data&doctype=json&version=1.1";

pub async fn youdao(word: &str) -> Result<Response, String> {
    let response = reqwest::get(format!("{url}&q={query}", url = YOUDAO_API, query = word))
        .await
        .map_err(|err| err.to_string())?;

    response
        .json::<Response>()
        .await
        .map_err(|err| err.to_string())
}
